//! Native platform services: file dialogs (rfd), filesystem, codecs.

use photocraft_codecs::{ChannelLayout, EncodeOptions, Image, SampleType as CS};
use photocraft_color::{ColorMode, SampleType};
use photocraft_doc::{Document, Layer, Size};
use photocraft_format::{Autosaver, RecoveryEntry};
use photocraft_geom::Rect;
use photocraft_ui_egui::{RecoveredDocument, RecoveryFailure, Services};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// Everything File › Open reads: PhotoCraft and Photoshop documents, flat images, and Photoshop
/// brushes (.abr) and gradients (.grd), which go to the preset libraries.
const OPEN_EXTS: &[&str] = &[
    "pcraft", "psd", "psb", "png", "jpg", "jpeg", "tif", "tiff", "webp", "gif", "bmp", "tga", "ico", "qoi", "exr", "hdr", "pbm", "pgm", "ppm", "pam", "pfm",
    "dng", "cr2", "cr3", "nef", "nrw", "arw", "pef", "orf", "rw2", "raf", "abr", "grd",
];

/// File › Save As formats: (filter name, extensions). The filter matching the suggested name's
/// extension comes first, so a .pcraft document saves as .pcraft by default and everything else
/// keeps defaulting to Photoshop.
const SAVE_FILTERS: &[(&str, &[&str])] =
    &[("Photoshop", &["psd", "psb"]), ("PhotoCraft", &["pcraft"]), ("PNG", &["png"]), ("JPEG", &["jpg"]), ("TIFF", &["tif"]), ("OpenEXR", &["exr"])];

/// [`SAVE_FILTERS`] with the one for `suggested`'s extension first.
fn save_filters(suggested: &str) -> Vec<(&'static str, &'static [&'static str])> {
    let ext = Path::new(suggested).extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    let mut v = SAVE_FILTERS.to_vec();
    if let Some(i) = v.iter().position(|(_, exts)| exts.contains(&ext.as_str())) {
        let f = v.remove(i);
        v.insert(0, f);
    }
    v
}

/// Per-user settings directory: `PHOTOCRAFT_CONFIG_DIR`, else `<exe dir>/PhotoCraftData` in
/// portable mode, else the platform convention. Everything the app persists lives under it; see
/// [`crate::app_dirs`].
pub fn config_dir() -> Option<PathBuf> {
    crate::app_dirs::config_dir()
}

pub fn prefs_file() -> Option<PathBuf> {
    config_dir().map(|d| d.join("preferences.json"))
}

/// The brush preset store (one file per preset group plus tip bitmaps; see
/// `photocraft_engine::preset_store`).
pub fn presets_dir() -> Option<PathBuf> {
    config_dir().map(|d| d.join("Presets"))
}

fn recovery_dir() -> Option<PathBuf> {
    config_dir().map(|d| d.join("Recovery"))
}

/// Write `bytes` crash-safely (temp file beside the target, fsync, rename, directory fsync; see
/// [`photocraft_format::atomic`]). Every document write (Save, Save As, Export, Save for Web) and
/// the preferences go through here, so a failed or interrupted save never destroys the old file.
fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    photocraft_format::atomic_write(path, bytes).map_err(|e| e.to_string())
}

fn recover_entries(dir: &Path) -> (Vec<Result<RecoveredDocument, RecoveryFailure>>, HashMap<String, RecoveryEntry>) {
    let mut outcomes = Vec::new();
    let mut recovery_entries = HashMap::new();
    let mut seen_keys = HashSet::new();
    for entry in photocraft_format::list_recovery(dir) {
        let key = entry.info.key.clone();
        if !photocraft_format::is_valid_recovery_key(&key) {
            continue;
        }
        if !seen_keys.insert(key.clone()) {
            outcomes.push(Err(RecoveryFailure { key, error: "duplicate recovery key".into() }));
            continue;
        }
        recovery_entries.insert(key.clone(), entry.clone());
        match photocraft_format::recover(&entry) {
            Ok(document) => outcomes.push(Ok(RecoveredDocument { key, original_path: entry.info.original_path.clone(), document })),
            Err(error) => outcomes.push(Err(RecoveryFailure { key, error: error.to_string() })),
        }
    }
    (outcomes, recovery_entries)
}

/// Avoid overwriting a persisted recovery key or live autosaver, including case-only path collisions on Windows.
fn available_recovery_key(id: u64, recovery_entries: &HashMap<String, RecoveryEntry>, savers: &HashMap<u64, Autosaver>) -> String {
    let mut key = format!("doc-{id}");
    while recovery_entries.keys().any(|existing| existing.eq_ignore_ascii_case(&key))
        || savers
            .values()
            .any(|saver| saver.bundle_path().file_stem().and_then(|stem| stem.to_str()).is_some_and(|existing| existing.eq_ignore_ascii_case(&key)))
    {
        key.push('_');
    }
    key
}

/// Reuse each document's existing key; allocate only when its first snapshot is requested.
fn recovery_key_for_autosave(
    id: u64,
    source_key: Option<&str>,
    recovery_entries: &HashMap<String, RecoveryEntry>,
    savers: &HashMap<u64, Autosaver>,
) -> Result<String, String> {
    if let Some(key) = source_key {
        if !recovery_entries.contains_key(key) {
            return Err(format!("unknown recovery entry {key}"));
        }
        return Ok(key.to_string());
    }
    if let Some(saver) = savers.get(&id) {
        return saver
            .bundle_path()
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_owned)
            .ok_or_else(|| format!("autosaver key for document {id} is not UTF-8"));
    }
    Ok(available_recovery_key(id, recovery_entries, savers))
}

pub fn native(automation: Option<photocraft_automation::AuthorizedWorkspace>) -> Services {
    let savers: Rc<RefCell<HashMap<u64, Autosaver>>> = Rc::default();
    let savers2 = savers.clone();
    let savers_for_recovery_discard = savers.clone();
    let recovery_entries: Rc<RefCell<HashMap<String, RecoveryEntry>>> = Rc::default();
    if let Some(dir) = recovery_dir() {
        recovery_entries.borrow_mut().extend(photocraft_format::list_recovery(&dir).into_iter().map(|entry| (entry.info.key.clone(), entry)));
    }
    let recovery_entries_for_autosave = recovery_entries.clone();
    let recovery_entries_for_discard = recovery_entries.clone();
    let recovery_entries_for_recover = recovery_entries.clone();
    let clip: Rc<RefCell<Option<arboard::Clipboard>>> = Rc::default();
    let automation_read = automation.clone().map(|workspace| {
        Box::new(move |path: &str| {
            let bytes = workspace.read(path).map_err(|error| error.to_string())?;
            let name = Path::new(path).file_name().and_then(|name| name.to_str()).unwrap_or(path).to_string();
            Ok((name, bytes))
        }) as photocraft_ui_egui::AutomationReadFn
    });
    let automation_write = automation.clone().map(|workspace| {
        Box::new(move |path: &str, bytes: &[u8]| workspace.write(path, bytes).map_err(|error| error.to_string())) as photocraft_ui_egui::AutomationWriteFn
    });
    let automation_command = automation.map(|_| {
        Box::new(|id: &str, params: &serde_json::Value| {
            photocraft_automation::workspace::authorize_desktop_engine_command(id, params).map_err(|error| error.to_string())
        }) as photocraft_ui_egui::AutomationCommandFn
    });
    Services {
        import: Some(Box::new(|name: &str, bytes: &[u8]| {
            crate::crash_guard::guard("Open", || photocraft_io::import(name, bytes).map(|r| (r.document, r.warnings)).map_err(|e| e.to_string()))
        })),
        export: Some(Box::new(|doc: &Document, path: &str, settings: &photocraft_ui_egui::ExportSettings| {
            let mut opts = photocraft_io::ExportOptions::default();
            if let Some(q) = settings.jpeg_quality {
                opts.encode.jpeg_quality = q;
            }
            crate::crash_guard::guard("Export", || photocraft_io::export(doc, path, &opts).map(|r| (r.bytes, r.warnings)).map_err(|e| e.to_string()))
        })),
        pick_open: Some(Box::new(|| {
            let path = rfd::FileDialog::new().add_filter("All Formats", OPEN_EXTS).add_filter("PhotoCraft", &["pcraft"]).pick_file()?;
            let bytes = std::fs::read(&path).ok()?;
            Some((path.to_string_lossy().to_string(), bytes))
        })),
        pick_save: Some(Box::new(|suggested: &str| {
            let p = std::path::Path::new(suggested);
            let mut d = rfd::FileDialog::new();
            for (name, exts) in save_filters(suggested) {
                d = d.add_filter(name, exts);
            }
            if let Some(name) = p.file_name() {
                d = d.set_file_name(name.to_string_lossy());
            }
            Some(d.save_file()?.to_string_lossy().to_string())
        })),
        write: Some(Box::new(|path: &str, bytes: &[u8]| write_atomic(Path::new(path), bytes))),
        automation_read,
        automation_write,
        automation_command,
        encode_png: Some(Box::new(|w, h, rgba| {
            let img = Image::from_u8(w, h, ChannelLayout::Rgba, rgba.to_vec()).map_err(|e| e.to_string())?;
            photocraft_codecs::encode(&img, photocraft_codecs::Format::Png, &EncodeOptions::default()).map_err(|e| e.to_string())
        })),
        inbox: None,
        open_url: Some(Box::new(|url: &str| open::that(url).map_err(|e| e.to_string()))),
        clipboard_set_image: Some({
            let clip = clip.clone();
            Box::new(move |w: u32, h: u32, px: &[u8]| {
                let mut slot = clip.try_borrow_mut().map_err(|_| "clipboard is busy".to_string())?;
                let cb = match slot.as_mut() {
                    Some(c) => c,
                    None => slot.insert(arboard::Clipboard::new().map_err(|e| e.to_string())?),
                };
                cb.set_image(arboard::ImageData { width: w as usize, height: h as usize, bytes: std::borrow::Cow::Borrowed(px) }).map_err(|e| e.to_string())
            })
        }),
        clipboard_get_image: Some({
            let clip = clip.clone();
            Box::new(move || {
                let mut slot = clip.try_borrow_mut().ok()?;
                let cb = match slot.as_mut() {
                    Some(c) => c,
                    None => slot.insert(arboard::Clipboard::new().ok()?),
                };
                let img = cb.get_image().ok()?;
                Some((img.width as u32, img.height as u32, img.bytes.into_owned()))
            })
        }),
        load_prefs: Some(Box::new(|| std::fs::read_to_string(prefs_file()?).ok())),
        save_prefs: Some(Box::new(|text: &str| write_atomic(&prefs_file().ok_or("no config directory")?, text.as_bytes()))),
        // Crash recovery: background incremental .pcraft saves into the recovery directory.
        autosave: Some(Box::new(move |doc: &Arc<Document>, revision: u64, path: Option<&str>, recovery_key: Option<&str>| {
            let dir = recovery_dir().ok_or("no config directory")?;
            let key = {
                let entries = recovery_entries_for_autosave.borrow();
                let active_savers = savers.borrow();
                recovery_key_for_autosave(doc.id.0, recovery_key, &entries, &active_savers)?
            };
            let mut map = savers.borrow_mut();
            let saver = map.entry(doc.id.0).or_insert_with(|| Autosaver::new(&dir, &key));
            if saver.bundle_path() != dir.join(format!("{key}.{}", photocraft_format::EXTENSION)) {
                return Err(format!("autosaver key does not match recovery entry {key}"));
            }
            saver.request(doc.clone(), revision, path.map(str::to_string), Default::default());
            Ok(())
        })),
        discard_autosave: Some(Box::new(move |id: u64| {
            if let Some(s) = savers2.borrow_mut().remove(&id) {
                let _ = s.discard();
            }
        })),
        discard_recovery: Some(Box::new(move |key: &str| {
            let entry = recovery_entries_for_discard.borrow().get(key).cloned().ok_or_else(|| format!("unknown recovery entry {key}"))?;
            let saver_id = savers_for_recovery_discard.borrow().iter().find_map(|(id, saver)| (saver.bundle_path() == entry.bundle).then_some(*id));
            if let Some(id) = saver_id
                && let Some(saver) = savers_for_recovery_discard.borrow_mut().remove(&id)
            {
                saver.discard().map_err(|error| error.to_string())?;
            }
            let dir = entry.bundle.parent().ok_or("recovery bundle has no parent")?;
            photocraft_format::discard_recovery(dir, &entry).map_err(|error| error.to_string())?;
            recovery_entries_for_discard.borrow_mut().remove(key);
            Ok(())
        })),
        recover: Some(Box::new(move || {
            let Some(dir) = recovery_dir() else { return Vec::new() };
            let (outcomes, entries) = recover_entries(&dir);
            *recovery_entries_for_recover.borrow_mut() = entries;
            outcomes
        })),
        append_text: Some(Box::new(|path: &str, text: &str| {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path).map_err(|e| e.to_string())?;
            f.write_all(text.as_bytes()).map_err(|e| e.to_string())
        })),
        // Set by main once the Apple-event handlers are connected (macOS).
        os_events: None,
        // Set by main, which starts loading the store before the window opens.
        preset_store: None,
    }
}

/// Flat-image import via photocraft-codecs (kept for reference/tests; the app uses photocraft-io).
#[allow(dead_code)]
pub fn import_flat(name: &str, bytes: &[u8]) -> Result<Document, String> {
    let img = photocraft_codecs::decode(bytes).map_err(|e| e.to_string())?;
    let (w, h) = (img.width(), img.height());
    let depth = match img.sample_type() {
        CS::U8 => SampleType::U8,
        CS::U16 => SampleType::U16,
        _ => SampleType::F32,
    };
    let gray = matches!(img.layout(), ChannelLayout::Gray | ChannelLayout::GrayA);
    let cmyk = matches!(img.layout(), ChannelLayout::Cmyk | ChannelLayout::CmykA);
    let mode = if gray {
        ColorMode::Grayscale
    } else if cmyk {
        ColorMode::Cmyk
    } else {
        ColorMode::Rgb
    };
    let target = match mode {
        ColorMode::Grayscale => ChannelLayout::GrayA,
        ColorMode::Cmyk => ChannelLayout::CmykA,
        _ => ChannelLayout::Rgba,
    };
    let sample = match depth {
        SampleType::U8 => CS::U8,
        SampleType::U16 => CS::U16,
        SampleType::F32 => CS::F32,
    };
    let conv = img.convert(target, sample);
    let stem = std::path::Path::new(name).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or(name.to_string());
    let mut doc = Document::new(stem, Size::new(w, h), mode, depth);
    doc.icc_profile = img.icc.clone().map(std::sync::Arc::new);
    if let Some((x, _)) = img.meta.dpi {
        doc.resolution_dpi = x;
    }
    let mut layer = Layer::raster("Background", doc.pixel_format());
    let data = conv.to_normalized();
    layer.surface_mut().ok_or("new raster layer has no pixels")?.write_region(Rect::from_xywh(0, 0, w, h), &data);
    doc.layers.push(layer);
    Ok(doc)
}

#[allow(dead_code)]
pub fn export_flat(doc: &Document, path: &str) -> Result<Vec<u8>, String> {
    let format = photocraft_codecs::from_extension(path).ok_or_else(|| format!("unknown file type for {path}"))?;
    let buf = photocraft_compose::flatten(doc);
    let (w, h) = (buf.rect.width(), buf.rect.height());
    let data: Vec<f32> = buf.px.iter().flat_map(|p| *p).collect();
    let img = match doc.depth {
        SampleType::U8 => Image::from_u8(w, h, ChannelLayout::Rgba, buf.to_rgba8().pixels),
        SampleType::U16 => Image::from_u16(w, h, ChannelLayout::Rgba, &data.iter().map(|v| (v.clamp(0.0, 1.0) * 65535.0 + 0.5) as u16).collect::<Vec<_>>()),
        SampleType::F32 => Image::from_f32(w, h, ChannelLayout::Rgba, &data),
    }
    .map_err(|e| e.to_string())?;
    let img = match &doc.icc_profile {
        Some(icc) if doc.mode == ColorMode::Rgb => img.with_icc(Some((**icc).clone())),
        _ => img,
    };
    photocraft_codecs::encode(&img, format, &EncodeOptions::default()).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn failed_recovery_entries_reserve_keys_case_insensitively() {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("photocraft-recovery-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for key in ["doc-1", "DOC-2"] {
            let saver = Autosaver::new(&dir, key);
            saver.request(Arc::new(Document::new(key, Size::new(8, 8), ColorMode::Rgb, SampleType::U8)), 1, None, Default::default());
            saver.flush().unwrap().unwrap();
        }
        std::fs::write(dir.join("DOC-2.pcraft").join("manifest.json"), b"{").unwrap();
        let sidecar = std::fs::read(dir.join("DOC-2.json")).unwrap();
        std::fs::write(dir.join("backup.json"), sidecar).unwrap();

        let (outcomes, entries) = recover_entries(&dir);

        assert_eq!(outcomes.len(), 3);
        assert!(outcomes.iter().any(|outcome| matches!(outcome, Ok(doc) if doc.key == "doc-1")));
        assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(failure) if failure.key == "DOC-2")).count(), 2);
        assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(failure) if failure.error == "duplicate recovery key")).count(), 1);
        assert!(entries.contains_key("doc-1"));
        assert!(entries.contains_key("DOC-2"), "failed source entries still own their recovery keys");
        assert_eq!(available_recovery_key(2, &entries, &HashMap::new()), "doc-2_");

        let saver = Autosaver::new(&dir, "DOC-3");
        let savers = HashMap::from([(4, saver)]);
        assert_eq!(available_recovery_key(3, &HashMap::new(), &savers), "doc-3_");
        assert_eq!(recovery_key_for_autosave(4, None, &HashMap::new(), &savers).unwrap(), "DOC-3");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
