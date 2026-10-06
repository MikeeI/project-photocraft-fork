//! Background autosave and crash recovery.

mod common;
use std::sync::Arc;

use common::*;
use photocraft_color::{ColorMode, SampleType};
use photocraft_format::*;

#[test]
fn autosave_then_recover() {
    let dir = temp_dir("recovery");
    let doc = Arc::new(rich_doc(ColorMode::Rgb, SampleType::U16));
    let mut saver = Autosaver::new(&dir, "doc-1");
    let bundle = saver.bundle_path();
    std::fs::write(&bundle, b"not a directory").unwrap();
    saver.request(doc.clone(), 7, Some("/work/a.pcraft".into()), SaveOptions::default()).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    let (failed_revision, failed) = loop {
        if let Some(result) = saver.take_results().into_iter().next() {
            break result;
        }
        assert!(std::time::Instant::now() < deadline, "autosave failure was not reported");
        std::thread::sleep(std::time::Duration::from_millis(1));
    };
    assert_eq!(failed_revision, 7);
    assert!(failed.is_err(), "a failed bundle write must not be acknowledged as persisted");
    std::fs::remove_file(&bundle).unwrap();
    saver.request(doc.clone(), 7, Some("/work/a.pcraft".into()), SaveOptions::default()).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    let (completed_revision, completed) = loop {
        if let Some(result) = saver.take_results().into_iter().next() {
            break result;
        }
        assert!(std::time::Instant::now() < deadline, "autosave completion was not reported");
        std::thread::sleep(std::time::Duration::from_millis(1));
    };
    assert_eq!(completed_revision, 7);
    assert!(completed.unwrap().tiles_written > 0);
    let r = saver.flush().expect("a save ran");
    let stats = r.unwrap();
    assert!(stats.tiles_written > 0);
    let entries = list_recovery(&dir);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].info.revision, 7);
    assert_eq!(entries[0].info.original_path.as_deref(), Some("/work/a.pcraft"));
    assert_eq!(entries[0].info.document_name, "Rich");
    assert_eq!(recover(&entries[0]).unwrap(), *doc);
    discard_recovery(&dir, &entries[0]).unwrap();
    assert!(list_recovery(&dir).is_empty());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn repeated_autosaves_are_incremental_and_coalesced() {
    let dir = temp_dir("coalesce");
    let saver = Autosaver::new(&dir, "k");
    let mut doc = rich_doc(ColorMode::Rgb, SampleType::U8);
    for i in 0..5 {
        let id = doc.layers[1].id;
        doc.layer_mut(id).unwrap().surface_mut().unwrap().write_pixel(i, 0, &[1.0, 0.0, 0.0, 1.0]);
        saver.request(Arc::new(doc.clone()), i as u64, None, SaveOptions::default()).unwrap();
    }
    saver.flush().unwrap().unwrap();
    let e = list_recovery(&dir);
    assert_eq!(e[0].info.revision, 4, "newest snapshot wins");
    assert_eq!(recover(&e[0]).unwrap(), doc);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn discard_removes_everything() {
    let dir = temp_dir("discard");
    let saver = Autosaver::new(&dir, "x y/z");
    saver.request(Arc::new(rich_doc(ColorMode::Grayscale, SampleType::U8)), 1, None, SaveOptions::default()).unwrap();
    let path = saver.bundle_path();
    assert!(path.file_name().unwrap().to_string_lossy().starts_with("x_y_z"));
    saver.discard().unwrap();
    assert!(list_recovery(&dir).is_empty());
    assert!(!path.exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn list_ignores_junk() {
    let dir = temp_dir("junk");
    std::fs::write(dir.join("bogus.json"), b"{}").unwrap();
    std::fs::write(dir.join("other.txt"), b"x").unwrap();
    assert!(list_recovery(&dir).is_empty());
    assert!(list_recovery(&dir.join("missing")).is_empty());
    std::fs::remove_dir_all(dir).unwrap();
}
