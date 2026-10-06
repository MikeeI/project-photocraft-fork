//! Indexed Color and Duotone documents in flat exports.

use photocraft_color::{Color, ColorMode, PixelFormat, SampleType};
use photocraft_doc::{AlphaChannel, ColorTable, Document, Duotone, DuotoneInk, Size};
use photocraft_raster::Surface;

fn doc(mode: ColorMode) -> Document {
    let mut d = Document::with_background("m", Size::new(4, 2), ColorMode::Rgb, SampleType::U8, Color::rgba(1.0, 0.0, 0.0, 1.0));
    d.mode = mode;
    d
}

fn add_nonpixel_channels(doc: &mut Document) {
    let mut saved = Surface::new(PixelFormat::GRAY8);
    saved.fill_rect(doc.bounds(), &[0.5]);
    doc.channels.push(AlphaChannel::new("Saved selection", saved));
    let mut quick_mask = Surface::new(PixelFormat::GRAY8);
    quick_mask.fill_rect(doc.bounds(), &[0.25]);
    doc.quick_mask = Some(AlphaChannel::new("Quick Mask", quick_mask));
}

fn assert_flat_channel_warnings(warnings: &[String]) {
    assert_eq!(warnings.iter().filter(|warning| warning.contains("saved channel")).count(), 1, "{warnings:?}");
    assert_eq!(warnings.iter().filter(|warning| warning.contains("Quick Mask")).count(), 1, "{warnings:?}");
}

#[test]
fn indexed_png_is_palette_png() {
    let mut d = doc(ColorMode::Indexed);
    d.color_table = Some(ColorTable { colors: vec![[0, 0, 0], [255, 0, 0]], transparent: None });
    add_nonpixel_channels(&mut d);
    let r = photocraft_io::export(&d, "x.png", &Default::default()).unwrap();
    assert_flat_channel_warnings(&r.warnings);
    // IHDR colour type 3 = indexed; a PLTE chunk follows.
    assert_eq!(r.bytes[25], 3);
    assert!(r.bytes.windows(4).any(|w| w == b"PLTE"));
    let back = photocraft_io::import("x.png", &r.bytes).unwrap().document;
    assert!(back.channels.is_empty());
    assert!(back.quick_mask.is_none());
    let px = back.layers[0].surface().unwrap().rgba(1, 1);
    assert_eq!(&px[..3], &[1.0, 0.0, 0.0]);
    // Other formats keep expanded pixels but drop document-owned channels.
    let j = photocraft_io::export(&d, "x.tif", &Default::default()).unwrap();
    assert_flat_channel_warnings(&j.warnings);
    let back = photocraft_io::import("x.tif", &j.bytes).unwrap().document;
    assert!(back.channels.is_empty());
    assert!(back.quick_mask.is_none());
}

#[test]
fn duotone_exports_the_inks_as_rgb() {
    // Mid gray background.
    let mut d = Document::with_background("m", Size::new(4, 2), ColorMode::Grayscale, SampleType::U8, Color::rgba(0.5, 0.5, 0.5, 1.0));
    d.mode = ColorMode::Duotone;
    d.duotone = Some(Duotone { inks: vec![DuotoneInk::new("Black", [0.0; 3]), DuotoneInk::new("Orange", [1.0, 0.5, 0.0])], psd_raw: None });
    add_nonpixel_channels(&mut d);
    let r = photocraft_io::export(&d, "x.png", &Default::default()).unwrap();
    assert_flat_channel_warnings(&r.warnings);
    assert!(r.warnings.iter().any(|w| w.contains("Duotone")), "{:?}", r.warnings);
    let back = photocraft_io::import("x.png", &r.bytes).unwrap().document;
    assert!(back.channels.is_empty());
    assert!(back.quick_mask.is_none());
    let px = back.layers[0].surface().unwrap().rgba(0, 0);
    assert!(px[0] > px[2] + 0.05, "warm: {px:?}");
}
