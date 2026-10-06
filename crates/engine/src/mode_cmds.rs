//! Image menu remnants: Image Rotation › Arbitrary, and the palette / ink modes — Mode › Indexed
//! Color, Color Table, Bitmap and Duotone.
//!
//! Indexed, Bitmap and Duotone documents keep expanded pixels (RGB 8-bit / Gray 8-bit / Gray, see
//! `Document::pixel_format`): Indexed and Bitmap quantise the flattened image in place and record
//! the palette (`Document::color_table`) or the 1-bit look; Duotone keeps the gray pixels and the
//! inks (`Document::duotone`), which display and flat export render through. Like Photoshop the
//! conversions flatten (Indexed, Bitmap) and require a grayscale source (Bitmap, Duotone).

use std::collections::HashSet;
use std::sync::Arc;

use photocraft_algo::quantize::{self, BitmapMethod, Dither, Forced, HalftoneShape, PaletteKind};
use photocraft_algo::transform::{Homography, Interp};
use photocraft_color::{ColorMode, PixelFormat, SampleType};
use photocraft_doc::adjust::CurvePoint;
use photocraft_doc::{ColorTable, Document, Duotone, DuotoneInk, IndexedPixels, Layer, LayerContent, LayerId, Size};
use photocraft_geom::{Affine, Rect, TILE_SIZE};
use photocraft_raster::Surface;
use serde_json::{Value, json};

use crate::commands::CommandSpec;
use crate::{EngineError, Result, Session};

const MISSING_INDEXED_PIXELS: &str = "indexed pixel identities are unavailable; reconvert this document to Indexed Color before editing its Color Table";

const INDEXED_APPEARANCE_EPSILON: f32 = 0.5 / 255.0;
const MAX_INDEXED_COLORS: usize = 256;

fn bad(cmd: &str, msg: impl Into<String>) -> EngineError {
    EngineError::BadParams { cmd: cmd.into(), msg: msg.into() }
}

fn num(p: &Value, key: &str, default: f32) -> f32 {
    p.get(key).and_then(Value::as_f64).map_or(default, |v| v as f32)
}

fn str_or<'a>(p: &'a Value, key: &str, default: &'a str) -> &'a str {
    p.get(key).and_then(Value::as_str).unwrap_or(default)
}

type Enabled = std::result::Result<(), String>;

fn has_doc(s: &Session) -> Enabled {
    s.active().map(|_| ()).ok_or_else(|| "no document open".into())
}

fn mode_is(s: &Session, modes: &[ColorMode], what: &str) -> Enabled {
    let d = s.active().ok_or("no document open")?;
    if modes.contains(&d.doc.mode) { Ok(()) } else { Err(format!("{what} (the document is {:?})", d.doc.mode)) }
}

// ---------- Image Rotation › Arbitrary ----------

/// Canvas size after rotating `size` by `deg` (the canvas grows to fit, as in Photoshop).
pub fn rotated_size(size: Size, deg: f64) -> Size {
    let (s, c) = deg.to_radians().sin_cos();
    let (w, h) = (f64::from(size.width), f64::from(size.height));
    let nw = (w * c.abs() + h * s.abs() - 1e-6).ceil().max(1.0);
    let nh = (w * s.abs() + h * c.abs() - 1e-6).ceil().max(1.0);
    Size::new(nw as u32, nh as u32)
}

fn rotate_arbitrary(s: &mut Session, p: &Value) -> Result<Value> {
    const CMD: &str = "image.rotation.arbitrary";
    let angle = f64::from(num(p, "angle", 0.0));
    if !angle.is_finite() || angle.abs() > 3600.0 {
        return Err(bad(CMD, "angle out of range"));
    }
    let deg = match str_or(p, "direction", "cw") {
        "cw" => angle,
        "ccw" => -angle,
        d => return Err(bad(CMD, format!("direction `{d}` (cw|ccw)"))),
    };
    if deg.rem_euclid(360.0).abs() < 1e-9 {
        return Ok(json!({"width": s.active().map(|d| d.doc.size.width), "height": s.active().map(|d| d.doc.size.height)}));
    }
    let bg = s.tools.background;
    let size = s.edit("Rotate Canvas", |doc, _| {
        let old = doc.size;
        let new = rotated_size(old, deg);
        let (sn, cs) = deg.to_radians().sin_cos();
        let (cx, cy) = (f64::from(old.width) / 2.0, f64::from(old.height) / 2.0);
        let (nx, ny) = (f64::from(new.width) / 2.0, f64::from(new.height) / 2.0);
        // Clockwise on screen (y down): x' = c·x − s·y, y' = s·x + c·y about the centres.
        let a = Affine { m: [cs, sn, -sn, cs, nx - (cs * cx - sn * cy), ny - (sn * cx + cs * cy)] };
        let h = Homography([a.m[0], a.m[2], a.m[4], a.m[1], a.m[3], a.m[5], 0.0, 0.0, 1.0]);
        let interp = Interp::parse(str_or(p, "interpolation", "bicubic"));
        doc.size = new;
        let canvas = doc.bounds();
        let fmt = doc.pixel_format();
        let dpi = doc.resolution_dpi;
        for l in doc.layers.iter_mut() {
            let background = l.name == "Background" && l.locks.position && matches!(l.content, LayerContent::Raster(_));
            if background && let Some(surf) = l.surface_mut() {
                // The Background stays a Background: rotated pixels over the background colour.
                let src = surf.content_bounds();
                let rotated = photocraft_algo::transform::warp_surface(surf, src, &h, interp);
                let mut base = Surface::new(fmt);
                let fill = photocraft_raster::from_rgba(&fmt, bg);
                base.write_region(canvas, &fill.repeat(canvas.width() as usize * canvas.height() as usize));
                crate::transform_cmds::composite_over(&mut base, &rotated);
                base.prune();
                *surf = base;
                if let Some(m) = l.mask.as_mut() {
                    m.surface = crate::transform_cmds::warp_gray(&m.surface, &h, interp);
                }
                if let Some(indexed) = &mut l.indexed_pixels {
                    let src = indexed.assignments().content_bounds();
                    *indexed.assignments_mut() = photocraft_algo::transform::warp_surface(indexed.assignments(), src, &h, Interp::Nearest);
                    *indexed.alpha_mut() = photocraft_algo::transform::warp_surface(indexed.alpha(), src, &h, Interp::Nearest);
                }
            } else {
                // Rotating the whole image moves locked layers too.
                let locks = l.locks;
                l.locks.position = false;
                l.locks.all = false;
                crate::transform_cmds::transform_layer(None, l, &h, Some(a), interp)?;
                l.locks = locks;
            }
            // What Free Transform leaves alone: unlinked vector masks, gradient angles, artboards…
            crate::canvas_geom::transform_layer_geometry(l, &a, false, dpi);
        }
        crate::canvas_geom::transform_doc_marks(doc, &a);
        for ch in doc.channels.iter_mut().chain(doc.quick_mask.as_mut()) {
            ch.surface = crate::transform_cmds::warp_gray(&ch.surface, &h, interp);
        }
        if let Some(sel) = &doc.selection {
            doc.selection = Some(crate::transform_cmds::warp_gray(sel, &h, Interp::Bilinear)).filter(|s| !s.content_bounds().is_empty());
        }
        // Type, shapes and smart objects re-render from their new geometry.
        crate::canvas_geom::refresh(doc, crate::canvas_geom::Refresh::All);
        doc.guides = Default::default();
        Ok(new)
    })?;
    Ok(json!({"width": size.width, "height": size.height}))
}

// ---------- flatten helpers ----------

/// The flattened image as straight RGBA (over white when `opaque`).
fn composite(doc: &Document, opaque: bool) -> Vec<[f32; 4]> {
    let buf = photocraft_compose::flatten(doc);
    if opaque { buf.over_background([1.0, 1.0, 1.0]).px } else { buf.px }
}

/// Replace the document's layers by one 8-bit layer holding `px` in `mode`'s storage format.
fn single_layer(doc: &mut Document, active: &mut Option<LayerId>, mode: ColorMode, px: &[[f32; 4]], name: &str, background: bool) {
    doc.mode = mode;
    doc.depth = SampleType::U8;
    let fmt = doc.pixel_format();
    let data: Vec<f32> = px.iter().flat_map(|q| photocraft_raster::from_rgba(&fmt, *q)).collect();
    let mut l = Layer::raster(name, fmt);
    if background {
        l.locks.transparency = true;
        l.locks.position = true;
    }
    // A new raster layer always has pixels.
    if let Some(surf) = l.surface_mut() {
        surf.write_region(doc.bounds(), &data);
    }
    *active = Some(l.id);
    doc.layers = vec![l];
    for ch in doc.channels.iter_mut() {
        let f = PixelFormat::new(ColorMode::Grayscale, SampleType::U8, false);
        ch.surface = ch.surface.convert(PixelFormat { alpha: ch.surface.format().alpha, ..f });
    }
    doc.quick_mask = None;
    doc.color_table = None;
    doc.duotone = None;
}

// ---------- Indexed Color / Color Table ----------

fn parse_forced(s: &str) -> Option<Forced> {
    Some(match s {
        "none" => Forced::None,
        "blackWhite" | "blackAndWhite" => Forced::BlackWhite,
        "primaries" => Forced::Primaries,
        "web" => Forced::Web,
        _ => return None,
    })
}

fn parse_dither(s: &str) -> Option<Dither> {
    Some(match s {
        "none" => Dither::None,
        "diffusion" => Dither::Diffusion,
        "pattern" => Dither::Pattern,
        "noise" => Dither::Noise,
        _ => return None,
    })
}

fn indexed_color(s: &mut Session, p: &Value) -> Result<Value> {
    const CMD: &str = "image.mode.indexedColor";
    let kind = PaletteKind::from_id(str_or(p, "palette", "selective"))
        .ok_or_else(|| bad(CMD, "palette: exact|systemMac|systemWindows|web|uniform|perceptual|selective|adaptive"))?;
    let forced = parse_forced(str_or(p, "forced", "blackWhite")).ok_or_else(|| bad(CMD, "forced: none|blackWhite|primaries|web"))?;
    let dither = parse_dither(str_or(p, "dither", "diffusion")).ok_or_else(|| bad(CMD, "dither: none|diffusion|pattern|noise"))?;
    let colors = num(p, "colors", 256.0).clamp(2.0, 256.0) as usize;
    let amount = num(p, "amount", 75.0).clamp(0.0, 100.0) / 100.0;
    let transparency = p.get("transparency").and_then(Value::as_bool).unwrap_or(true);
    let (pal_len, transparent) = s.edit("Indexed Color", |doc, active| {
        let mut px = composite(doc, !transparency);
        let has_clear = transparency && px.iter().any(|q| q[3] < 0.5);
        let mut pal = quantize::build_palette(&px, kind, if has_clear { colors.saturating_sub(1).max(2) } else { colors }, forced).map_err(|e| bad(CMD, e))?;
        let transparent = if has_clear && pal.len() < 256 {
            pal.push([255, 255, 255]);
            Some(pal.len() - 1)
        } else {
            None
        };
        let mut source_alpha = Vec::new();
        source_alpha.try_reserve_exact(px.len()).map_err(|e| EngineError::Other(format!("could not allocate indexed alpha data: {e}")))?;
        source_alpha.extend(px.iter().map(|q| q[3]));
        let w = doc.size.width as usize;
        let indices = quantize::quantize(&mut px, w, &pal, dither, amount, transparent);
        if indices.len() != source_alpha.len() {
            return Err(EngineError::Other("quantizer returned an invalid indexed pixel count".into()));
        }
        single_layer(doc, active, ColorMode::Indexed, &px, if transparent.is_some() { "Index" } else { "Background" }, transparent.is_none());
        let bounds = doc.bounds();
        let assignment_capacity = indices.len().checked_mul(2).ok_or_else(|| EngineError::Other("indexed assignment map is too large".into()))?;
        let mut assignments = Vec::new();
        assignments.try_reserve_exact(assignment_capacity).map_err(|e| EngineError::Other(format!("could not allocate indexed assignments: {e}")))?;
        let mut alpha = Vec::new();
        alpha.try_reserve_exact(source_alpha.len()).map_err(|e| EngineError::Other(format!("could not allocate indexed alpha map: {e}")))?;
        for (index, original_alpha) in indices.iter().zip(source_alpha) {
            assignments.extend([f32::from(*index) / 255.0, 1.0]);
            alpha.push(if transparent == Some(usize::from(*index)) { 1.0 } else { original_alpha });
        }
        let layer = doc.layers.first_mut().ok_or_else(|| EngineError::Other("Indexed Color did not create its raster layer".into()))?;
        let mut indexed = IndexedPixels::new();
        indexed.assignments_mut().write_region(bounds, &assignments);
        indexed.alpha_mut().write_region(bounds, &alpha);
        layer.indexed_pixels = Some(indexed);
        doc.color_table = Some(ColorTable { colors: pal.clone(), transparent: transparent.map(|t| t as u8) });
        Ok((pal.len(), transparent))
    })?;
    Ok(json!({"colors": pal_len, "transparentIndex": transparent}))
}

fn nearest_palette_index(table: &ColorTable, color: [f32; 3], prefer_visible: bool) -> Option<usize> {
    let mut best: Option<(f32, usize)> = None;
    for (index, entry) in table.colors.iter().enumerate() {
        if prefer_visible && table.transparent == Some(index as u8) {
            continue;
        }
        let distance: f32 = (0..3).map(|channel| (color[channel] - f32::from(entry[channel]) / 255.0).powi(2)).sum();
        if best.is_none_or(|(current, _)| distance < current) {
            best = Some((distance, index));
        }
    }
    best.map(|(_, index)| index)
}

/// Retire palette maps outside Indexed Color; while active, repair changed pixels and preserve
/// exact identity for unchanged duplicate-color samples and new raster layers.
pub(crate) fn reconcile_indexed_pixels(before: &Document, after: &mut Document) -> Result<()> {
    if before.mode == ColorMode::Indexed && after.mode != ColorMode::Indexed {
        // Index identity is authoritative only while palette indexing is active.
        clear_indexed_layers(&mut after.layers);
        return Ok(());
    }
    if before.mode != ColorMode::Indexed || after.mode != ColorMode::Indexed {
        return Ok(());
    }
    let table = after.color_table.clone().ok_or_else(|| EngineError::Other("Indexed Color document has no color table".into()))?;
    validate_indexed_table(&table)?;
    let bounds = union_bounds(before.bounds(), after.bounds());
    reconcile_indexed_layers(&mut after.layers, before, &table, bounds)
}

pub(crate) fn validate_indexed_table(table: &ColorTable) -> Result<()> {
    if table.colors.is_empty() {
        return Err(EngineError::Other("Indexed Color document has an empty color table".into()));
    }
    if table.colors.len() > MAX_INDEXED_COLORS {
        return Err(EngineError::Other(format!("Indexed Color table exceeds {MAX_INDEXED_COLORS} entries")));
    }
    if table.transparent.is_some_and(|index| usize::from(index) >= table.colors.len()) {
        return Err(EngineError::Other("Indexed Color transparency index exceeds its color table".into()));
    }
    Ok(())
}

fn clear_indexed_layers(layers: &mut [Layer]) {
    for layer in layers {
        layer.indexed_pixels = None;
        if let Some(children) = layer.children_mut() {
            clear_indexed_layers(children);
        }
    }
}

fn union_bounds(a: Rect, b: Rect) -> Rect {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    Rect::new(a.x0.min(b.x0), a.y0.min(b.y0), a.x1.max(b.x1), a.y1.max(b.y1))
}

fn reconcile_indexed_layers(layers: &mut [Layer], before: &Document, table: &ColorTable, bounds: Rect) -> Result<()> {
    for layer in layers {
        match &mut layer.content {
            LayerContent::Raster(surface) => {
                let old_raster = before.layer(layer.id).and_then(|old| match &old.content {
                    LayerContent::Raster(old_surface) => Some((old_surface, old.indexed_pixels.as_ref())),
                    _ => None,
                });
                if let Some((old_surface, Some(old_indexed))) = old_raster {
                    if layer.indexed_pixels.is_none() {
                        layer.indexed_pixels = Some(old_indexed.clone());
                    }
                    if let Some(indexed) = layer.indexed_pixels.as_mut() {
                        reconcile_indexed_samples(old_surface, surface, old_indexed, indexed, table, bounds)?;
                    }
                } else if old_raster.is_none() && layer.indexed_pixels.is_none() {
                    layer.indexed_pixels = Some(quantize_indexed_surface(surface, table)?);
                }
            }
            LayerContent::Group(group) => {
                layer.indexed_pixels = None;
                reconcile_indexed_layers(&mut group.children, before, table, bounds)?;
            }
            _ => layer.indexed_pixels = None,
        }
    }
    Ok(())
}

// New raster samples are quantized into the active palette before their IDs become authoritative.
pub(crate) fn quantize_indexed_surface(surface: &mut Surface, table: &ColorTable) -> Result<IndexedPixels> {
    let mut indexed = IndexedPixels::new();
    validate_indexed_table(table)?;
    let bounds = surface.content_bounds();
    if bounds.is_empty() {
        return Ok(indexed);
    }
    let format = surface.format();
    let mut raw = [0.0; 8];
    for y in bounds.y0..bounds.y1 {
        for x in bounds.x0..bounds.x1 {
            surface.read_pixel(x, y, &mut raw[..format.channels()]);
            let rgba = photocraft_raster::to_rgba(&format, &raw[..format.channels()]);
            let color = [rgba[0], rgba[1], rgba[2]];
            let index =
                nearest_palette_index(table, color, rgba[3] > 0.0).ok_or_else(|| EngineError::Other("indexed raster has no visible palette entry".into()))?;
            let index_u8 = index as u8;
            if !indexed.set_sample(x, y, index_u8, rgba[3]) {
                return Err(EngineError::Other("indexed raster contains non-finite alpha".into()));
            }
            let entry = table.colors[index];
            let alpha = if table.transparent == Some(index_u8) { 0.0 } else { rgba[3] };
            let expanded =
                photocraft_raster::from_rgba(&format, [f32::from(entry[0]) / 255.0, f32::from(entry[1]) / 255.0, f32::from(entry[2]) / 255.0, alpha]);
            surface.write_pixel(x, y, &expanded);
        }
    }
    Ok(indexed)
}

fn collect_changed_tiles(before: &Surface, after: &Surface, bounds: Rect, changed: &mut HashSet<(i32, i32)>) {
    if before.format() != after.format() || before.default_pixel() != after.default_pixel() {
        changed.extend(bounds.tiles().map(|tile| (tile.tx, tile.ty)));
    }
    for (coord, tile) in after.tiles() {
        if !before.tile(*coord).is_some_and(|old| Arc::ptr_eq(old, tile)) {
            changed.insert((coord.tx, coord.ty));
        }
    }
    for (coord, _) in before.tiles() {
        if after.tile(*coord).is_none() {
            changed.insert((coord.tx, coord.ty));
        }
    }
}

fn clipped_tile_bounds(tx: i32, ty: i32, bounds: Rect) -> Option<Rect> {
    let tile_size = i64::from(TILE_SIZE);
    let x0 = (i64::from(tx) * tile_size).max(i64::from(bounds.x0));
    let y0 = (i64::from(ty) * tile_size).max(i64::from(bounds.y0));
    let x1 = (i64::from(tx) * tile_size + tile_size).min(i64::from(bounds.x1));
    let y1 = (i64::from(ty) * tile_size + tile_size).min(i64::from(bounds.y1));
    if x0 >= x1 || y0 >= y1 {
        return None;
    }
    Some(Rect::new(i32::try_from(x0).ok()?, i32::try_from(y0).ok()?, i32::try_from(x1).ok()?, i32::try_from(y1).ok()?))
}

fn reconcile_indexed_samples(
    before_surface: &Surface,
    after_surface: &mut Surface,
    before_indexed: &IndexedPixels,
    after_indexed: &mut IndexedPixels,
    table: &ColorTable,
    bounds: Rect,
) -> Result<()> {
    let mut changed = HashSet::new();
    collect_changed_tiles(before_surface, after_surface, bounds, &mut changed);
    collect_changed_tiles(before_indexed.assignments(), after_indexed.assignments(), bounds, &mut changed);
    collect_changed_tiles(before_indexed.alpha(), after_indexed.alpha(), bounds, &mut changed);
    if changed.is_empty() {
        return Ok(());
    }
    let before_format = before_surface.format();
    let after_format = after_surface.format();
    let same_format = before_format == after_format;
    let before_channels = before_format.channels();
    let after_channels = after_format.channels();
    let mut before_raw = [0.0; 8];
    let mut after_raw = [0.0; 8];
    for (tx, ty) in changed {
        let Some(tile_bounds) = clipped_tile_bounds(tx, ty, bounds) else {
            continue;
        };
        for y in tile_bounds.y0..tile_bounds.y1 {
            for x in tile_bounds.x0..tile_bounds.x1 {
                before_surface.read_pixel(x, y, &mut before_raw[..before_channels]);
                after_surface.read_pixel(x, y, &mut after_raw[..after_channels]);
                let surface_changed = !same_format || before_raw[..before_channels] != after_raw[..after_channels];
                if let Some((index, base_alpha)) = after_indexed.sample(x, y) {
                    let Some(entry) = table.colors.get(usize::from(index)) else {
                        return Err(EngineError::Other(format!("indexed pixel assignment {index} exceeds its color table")));
                    };
                    let actual = photocraft_raster::to_rgba(&after_format, &after_raw[..after_channels]);
                    let expected_alpha = if table.transparent == Some(index) { 0.0 } else { base_alpha };
                    let matches =
                        actual[..3].iter().zip(entry).all(|(actual, expected)| (*actual - f32::from(*expected) / 255.0).abs() <= INDEXED_APPEARANCE_EPSILON)
                            && (actual[3] - expected_alpha).abs() <= INDEXED_APPEARANCE_EPSILON;
                    if matches {
                        continue;
                    }
                    if !surface_changed {
                        return Err(EngineError::Other("indexed pixel sidecar diverged from unchanged raster data".into()));
                    }
                } else if !surface_changed {
                    continue;
                }
                let actual = photocraft_raster::to_rgba(&after_format, &after_raw[..after_channels]);
                let color = [actual[0], actual[1], actual[2]];
                let index = nearest_palette_index(table, color, actual[3] > 0.0)
                    .ok_or_else(|| EngineError::Other("indexed raster has no visible palette entry".into()))?;
                let index_u8 = index as u8;
                if !after_indexed.set_sample(x, y, index_u8, actual[3]) {
                    return Err(EngineError::Other("indexed raster contains non-finite alpha".into()));
                }
                // Indexed RGB is a projection of palette identity, so edits must snap to that entry.
                let entry = table.colors[index];
                let alpha = if table.transparent == Some(index_u8) { 0.0 } else { actual[3] };
                let expanded =
                    photocraft_raster::from_rgba(&after_format, [f32::from(entry[0]) / 255.0, f32::from(entry[1]) / 255.0, f32::from(entry[2]) / 255.0, alpha]);
                after_surface.write_pixel(x, y, &expanded);
            }
        }
    }
    Ok(())
}

fn apply_color_table_layers(layers: &mut [Layer], old: &ColorTable, table: &ColorTable, cmd: &str) -> Result<()> {
    for layer in layers {
        match &mut layer.content {
            LayerContent::Raster(surface) => {
                let Some(indexed) = layer.indexed_pixels.as_mut() else {
                    return Err(bad(cmd, MISSING_INDEXED_PIXELS));
                };
                let bounds = indexed.assignments().content_bounds();
                if bounds.is_empty() {
                    continue;
                }
                let format = surface.format();
                let channels = format.channels();
                let mut pixels = surface.read_region(bounds);
                let mut assignments = indexed.assignments().read_region(bounds);
                let alpha = indexed.alpha().read_region(bounds);
                let mut assignment_changed = false;
                let (assignment_pairs, _) = assignments.as_chunks_mut::<2>();
                for ((pixel, assignment), &base_alpha) in pixels.chunks_exact_mut(channels).zip(assignment_pairs).zip(&alpha) {
                    if assignment[1] < 0.5 {
                        continue;
                    }
                    let index = (assignment[0] * 255.0).round().clamp(0.0, 255.0) as usize;
                    let old_color = old.colors.get(index).ok_or_else(|| bad(cmd, format!("invalid saved palette index {index}")))?;
                    let index = if index < table.colors.len() {
                        index
                    } else {
                        let color = old_color.map(|v| f32::from(v) / 255.0);
                        let replacement = nearest_palette_index(table, color, base_alpha > 0.0)
                            .ok_or_else(|| bad(cmd, "the new color table has no visible entry for an assigned pixel"))?;
                        assignment[0] = replacement as f32 / 255.0;
                        assignment_changed = true;
                        replacement
                    };
                    let color = table.colors[index].map(|v| f32::from(v) / 255.0);
                    let output_alpha = if table.transparent == Some(index as u8) { 0.0 } else { base_alpha };
                    let mut encoded = [0.0f32; 8];
                    let count = photocraft_raster::from_rgba_into(&format, [color[0], color[1], color[2], output_alpha], &mut encoded);
                    pixel.copy_from_slice(&encoded[..count]);
                }
                surface.write_region(bounds, &pixels);
                surface.prune();
                if assignment_changed {
                    indexed.assignments_mut().write_region(bounds, &assignments);
                    indexed.assignments_mut().prune();
                }
            }
            LayerContent::Group(group) => apply_color_table_layers(&mut group.children, old, table, cmd)?,
            _ => {}
        }
    }
    Ok(())
}

fn hex(c: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

fn parse_color(v: &Value) -> Option<[u8; 3]> {
    match v {
        Value::String(s) => {
            let s = s.trim_start_matches('#');
            let b = |i: usize| u8::from_str_radix(s.get(i..i + 2)?, 16).ok();
            Some([b(0)?, b(2)?, b(4)?])
        }
        Value::Array(a) if a.len() >= 3 => {
            let c = |i: usize| a[i].as_f64().map(|x| x.round().clamp(0.0, 255.0) as u8);
            Some([c(0)?, c(1)?, c(2)?])
        }
        _ => None,
    }
}

/// Built-in Color Table presets.
pub fn preset_table(name: &str) -> Option<Vec<[u8; 3]>> {
    let ramp = |f: &dyn Fn(f32) -> [f32; 3]| (0..256).map(|i| f(i as f32 / 255.0).map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)).collect();
    Some(match name {
        "grayscale" => ramp(&|t| [t; 3]),
        // Black → red → yellow → white, the incandescence scale.
        "blackBody" => ramp(&|t| [(t * 3.0).min(1.0), (t * 3.0 - 1.0).clamp(0.0, 1.0), (t * 3.0 - 2.0).clamp(0.0, 1.0)]),
        "spectrum" => ramp(&|t| {
            let h = t * 300.0 / 60.0;
            let x = 1.0 - (h % 2.0 - 1.0).abs();
            match h as i32 {
                0 => [1.0, x, 0.0],
                1 => [x, 1.0, 0.0],
                2 => [0.0, 1.0, x],
                3 => [0.0, x, 1.0],
                _ => [x, 0.0, 1.0],
            }
        }),
        "systemMac" | "system" => quantize::mac_palette(),
        "systemWindows" => quantize::windows_palette(),
        "web" => quantize::web_palette(),
        _ => return None,
    })
}

/// Image › Mode › Color Table: replace entries (pixels keep their index, so their colour follows
/// the table, as in Photoshop) and set the transparent entry. No changes: report the table.
fn color_table(s: &mut Session, p: &Value) -> Result<Value> {
    const CMD: &str = "image.mode.colorTable";
    let active = s.active().ok_or(EngineError::NoDocument)?;
    let cur = active.doc.color_table.clone().ok_or_else(|| EngineError::Other("the document has no color table".into()))?;
    if cur.colors.is_empty() {
        return Err(bad(CMD, "the document's color table has no entries"));
    }
    if active.doc.walk().into_iter().any(|(_, _, layer)| matches!(&layer.content, LayerContent::Raster(_)) && layer.indexed_pixels.is_none()) {
        return Err(bad(CMD, MISSING_INDEXED_PIXELS));
    }
    let mut table = cur.clone();
    match str_or(p, "table", "custom") {
        "custom" => {}
        name => table.colors = preset_table(name).ok_or_else(|| bad(CMD, format!("unknown table `{name}`")))?,
    }
    if let Some(a) = p.get("colors").and_then(Value::as_array) {
        let c: Option<Vec<[u8; 3]>> = a.iter().map(parse_color).collect();
        let c = c.ok_or_else(|| bad(CMD, "colors: [\"#rrggbb\" | [r, g, b], …]"))?;
        if c.is_empty() || c.len() > 256 {
            return Err(bad(CMD, "a color table has 1 to 256 entries"));
        }
        table.colors = c;
    }
    if let Some(e) = p.get("entries").and_then(Value::as_object) {
        for (k, v) in e {
            let i: usize = k.parse().map_err(|_| bad(CMD, format!("entry index `{k}`")))?;
            let c = parse_color(v).ok_or_else(|| bad(CMD, format!("entry {i}: bad colour")))?;
            if i >= table.colors.len() {
                return Err(bad(CMD, format!("entry {i} is past the table ({} entries)", table.colors.len())));
            }
            table.colors[i] = c;
        }
    }
    match p.get("transparent") {
        Some(Value::Null) => table.transparent = None,
        Some(v) => table.transparent = v.as_u64().filter(|&i| (i as usize) < table.colors.len()).map(|i| i as u8).or(table.transparent),
        None => {}
    }
    if table.transparent.is_some_and(|i| usize::from(i) >= table.colors.len()) {
        table.transparent = None;
    }
    let report = |t: &ColorTable| json!({"colors": t.colors.iter().map(|c| hex(*c)).collect::<Vec<_>>(), "transparent": t.transparent});
    if table == cur {
        return Ok(report(&cur));
    }
    let out = report(&table);
    s.edit("Color Table", |doc, _| {
        apply_color_table_layers(&mut doc.layers, &cur, &table, CMD)?;
        doc.color_table = Some(table.clone());
        Ok(())
    })?;
    Ok(out)
}

// ---------- Bitmap ----------

fn bitmap(s: &mut Session, p: &Value) -> Result<Value> {
    const CMD: &str = "image.mode.bitmap";
    let dpi = s.active().ok_or(EngineError::NoDocument)?.doc.resolution_dpi.max(1.0);
    let method = match str_or(p, "method", "diffusion") {
        "threshold" | "threshold50" => BitmapMethod::Threshold,
        "pattern" => BitmapMethod::Pattern,
        "diffusion" => BitmapMethod::Diffusion,
        "halftone" => {
            let freq = num(p, "frequency", 53.0).clamp(1.0, 999.0);
            BitmapMethod::Halftone { cell: (dpi / freq).max(2.0), angle: num(p, "angle", 45.0), shape: HalftoneShape::from_id(str_or(p, "shape", "round")) }
        }
        m => {
            return Err(bad(CMD, format!("method `{m}` (threshold|pattern|diffusion|halftone)")));
        }
    };
    s.edit("Bitmap", |doc, active| {
        let px = composite(doc, true);
        let mut gray: Vec<f32> = px.iter().map(|q| photocraft_color::convert::rgb_to_gray([q[0], q[1], q[2]])).collect();
        quantize::to_bitmap(&mut gray, doc.size.width as usize, method);
        let out: Vec<[f32; 4]> = gray.iter().map(|g| [*g, *g, *g, 1.0]).collect();
        single_layer(doc, active, ColorMode::Bitmap, &out, "Background", true);
        Ok(())
    })?;
    Ok(Value::Null)
}

// ---------- Duotone ----------

/// Default inks per type: black first, then warm brown, gold and slate blue (generic names;
/// Photoshop ships Pantone presets, which we don't reproduce).
fn default_inks(n: usize) -> Vec<DuotoneInk> {
    let all = [("Black", [0.0, 0.0, 0.0]), ("Warm Brown", [0.62, 0.38, 0.18]), ("Gold", [0.86, 0.68, 0.18]), ("Slate Blue", [0.27, 0.38, 0.58])];
    all.iter().take(n).map(|(name, c)| DuotoneInk::new(*name, *c)).collect()
}

fn parse_inks(cmd: &str, v: &Value, n: usize) -> Result<Vec<DuotoneInk>> {
    let a = v.as_array().ok_or_else(|| bad(cmd, "inks: [{\"name\",\"color\":\"#rrggbb\",\"curve\":[[in,out],…] (0..100)}, …]"))?;
    let defaults = default_inks(n.max(a.len()).min(4));
    let mut out = Vec::new();
    for (i, ink) in a.iter().take(4).enumerate() {
        let mut d = defaults.get(i).cloned().unwrap_or_else(|| DuotoneInk::new(format!("Ink {}", i + 1), [0.0; 3]));
        if let Some(name) = ink.get("name").and_then(Value::as_str) {
            d.name = name.to_string();
        }
        if let Some(c) = ink.get("color").and_then(parse_color) {
            d.color = c.map(|v| f32::from(v) / 255.0);
        }
        if let Some(pts) = ink.get("curve").and_then(Value::as_array) {
            let mut curve: Vec<CurvePoint> =
                pts.iter().filter_map(|p| Some(CurvePoint { input: p.get(0)?.as_f64()? as f32 / 100.0, output: p.get(1)?.as_f64()? as f32 / 100.0 })).collect();
            curve.sort_by(|a, b| a.input.total_cmp(&b.input));
            if curve.len() >= 2 {
                d.curve = curve;
            }
        }
        out.push(d);
    }
    Ok(out)
}

fn duotone(s: &mut Session, p: &Value) -> Result<Value> {
    const CMD: &str = "image.mode.duotone";
    let n = match str_or(p, "type", "duotone") {
        "monotone" => 1,
        "duotone" => 2,
        "tritone" => 3,
        "quadtone" => 4,
        t => {
            return Err(bad(CMD, format!("type `{t}` (monotone|duotone|tritone|quadtone)")));
        }
    };
    let mut inks = match p.get("inks") {
        Some(v) => parse_inks(CMD, v, n)?,
        None => default_inks(n),
    };
    if inks.len() < n {
        inks.extend(default_inks(n).into_iter().skip(inks.len()));
    }
    inks.truncate(n);
    let names: Vec<String> = inks.iter().map(|i| i.name.clone()).collect();
    s.edit("Duotone", |doc, _| {
        doc.mode = ColorMode::Duotone;
        doc.duotone = Some(Duotone { inks, psd_raw: None });
        doc.color_table = None;
        Ok(())
    })?;
    Ok(json!({"inks": names}))
}

macro_rules! spec {
    ($id:literal, $label:literal, [$($m:literal),*], $params:literal, $en:expr, $run:expr) => {
        CommandSpec { id: $id, label: $label, menu: &[$($m),*], shortcut: None, params: $params, enabled: $en, run: $run, journal: true }
    };
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        spec!(
            "image.rotation.arbitrary",
            "Arbitrary…",
            ["Image", "Image Rotation"],
            r##"{"angle":-359.99..359.99=0,"direction":"cw|ccw"="cw","interpolation":"bicubic|bilinear|nearest"="bicubic"}"##,
            has_doc,
            rotate_arbitrary
        ),
        spec!(
            "image.mode.indexedColor",
            "Indexed Color…",
            ["Image", "Mode"],
            r##"{"palette":"selective|perceptual|adaptive|exact|systemMac|systemWindows|web|uniform"="selective","colors":2..256=256,"forced":"blackWhite|none|primaries|web"="blackWhite","transparency":bool=true,"dither":"diffusion|none|pattern|noise"="diffusion","amount":0..100=75} (flattens)"##,
            |s| mode_is(
                s,
                &[ColorMode::Rgb, ColorMode::Grayscale, ColorMode::Indexed, ColorMode::Duotone, ColorMode::Cmyk, ColorMode::Lab],
                "Indexed Color needs an RGB or Grayscale image"
            ),
            indexed_color
        ),
        spec!(
            "image.mode.colorTable",
            "Color Table…",
            ["Image", "Mode"],
            r##"{"table":"custom|blackBody|grayscale|spectrum|systemMac|systemWindows|web"="custom","colors":json,"entries":json,"transparent":json} (colors: ["#rrggbb", …]; entries: {"index": "#rrggbb"}; transparent: index|null)"##,
            |s| mode_is(s, &[ColorMode::Indexed], "Color Table needs an Indexed Color image"),
            color_table
        ),
        spec!(
            "image.mode.bitmap",
            "Bitmap…",
            ["Image", "Mode"],
            r##"{"method":"diffusion|threshold|pattern|halftone"="diffusion","frequency":1..999=53,"angle":-180..180=45,"shape":"round|ellipse|line|square|diamond|cross"="round"} (halftone frequency in lines/inch; flattens)"##,
            |s| mode_is(s, &[ColorMode::Grayscale], "Bitmap needs a Grayscale image (Image › Mode › Grayscale first)"),
            bitmap
        ),
        spec!(
            "image.mode.duotone",
            "Duotone…",
            ["Image", "Mode"],
            r##"{"type":"duotone|monotone|tritone|quadtone"="duotone","inks":json} (inks: [{"name","color":"#rrggbb","curve":[[in,out],…] in 0..100}, …])"##,
            |s| mode_is(s, &[ColorMode::Grayscale, ColorMode::Duotone], "Duotone needs a Grayscale image (Image › Mode › Grayscale first)"),
            duotone
        ),
    ]
}

/// The gradient-map layer that renders Duotone inks over the gray composite.
pub fn duotone_display_layer(d: &Duotone) -> Layer {
    Layer::new("Duotone", LayerContent::Adjustment(d.display_adjustment()))
}

/// A copy of `doc` that displays like the printed result (Duotone inks), or None when the
/// document displays as stored.
pub fn display_document(doc: &Document) -> Option<Document> {
    let d = doc.duotone.as_ref().filter(|_| doc.mode == ColorMode::Duotone)?;
    let mut out = doc.clone();
    out.layers.push(duotone_display_layer(d));
    Some(out)
}

#[cfg(test)]
mod tests;
