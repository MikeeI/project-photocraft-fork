//! Downsampled proxy documents: every pixel surface scaled down by an integer factor `k`, so a
//! composite of the proxy approximates the document's composite at 1/k resolution for ~1/k² of
//! the work. Used for interactive previews and for thumbnails of large documents (navigator,
//! channel thumbnails, histograms, file previews), where a full-resolution composite of a 200 MP
//! document would cost seconds and gigabytes.

use photocraft_doc::{Document, Layer, LayerContent, Size};
use photocraft_raster::Surface;

/// Nearest-neighbour downsample of a surface by integer factor `k` (document coordinates / k).
pub fn downsample(s: &Surface, k: u32) -> Surface {
    s.downsample_encoded(k)
}

fn shrink_layer(l: &mut Layer, k: u32) {
    if let Some(m) = &mut l.mask {
        m.surface = downsample(&m.surface, k);
    }
    if let Some(fc) = &mut l.fill_cache {
        fc.surface = downsample(&fc.surface, k);
    }
    match &mut l.content {
        LayerContent::Raster(s) => *s = downsample(s, k),
        LayerContent::Group(g) => {
            for c in &mut g.children {
                shrink_layer(c, k);
            }
        }
        LayerContent::Text(t) => t.cache = t.cache.as_ref().map(|s| downsample(s, k)),
        LayerContent::Shape(sh) => sh.cache = sh.cache.as_ref().map(|s| downsample(s, k)),
        LayerContent::Smart(so) => so.cache = so.cache.as_ref().map(|s| downsample(s, k)),
        LayerContent::Adjustment(_) | LayerContent::Fill(_) => {}
    }
}

/// A copy of `doc` scaled down by `k` (same layer ids, so adjustments can be swapped in).
pub fn proxy_document(doc: &Document, k: u32) -> Document {
    let mut p = doc.clone();
    if k <= 1 {
        return p;
    }
    p.size = Size::new(doc.size.width.div_ceil(k), doc.size.height.div_ceil(k));
    for l in &mut p.layers {
        shrink_layer(l, k);
    }
    // Spot channels are printed under Multichannel composites.
    for c in &mut p.channels {
        c.surface = downsample(&c.surface, k);
    }
    p.selection = None;
    p
}

/// Whether a proxy composite is a faithful reduction of `doc`: layer effects (whose sizes are in
/// document pixels) and vector masks (paths in document coordinates) would not scale with it.
pub fn proxy_faithful(doc: &Document) -> bool {
    doc.walk().iter().all(|(_, _, l)| !crate::effects::has_effects(l) && l.vector_mask.as_ref().is_none_or(|v| !v.enabled))
}

#[cfg(test)]
mod tests {
    use super::*;
    use photocraft_color::{Color, ColorMode, PixelFormat, SampleType};
    use photocraft_geom::Rect;

    #[test]
    fn downsample_picks_every_kth_pixel() {
        let mut s = Surface::new(PixelFormat::RGBA8);
        s.fill_rect(Rect::new(0, 0, 8, 8), &[1.0, 0.0, 0.0, 1.0]);
        s.write_pixel(4, 4, &[0.0, 0.0, 1.0, 1.0]);
        let d = downsample(&s, 4);
        assert_eq!(d.pixel(0, 0), vec![1.0, 0.0, 0.0, 1.0]);
        assert_eq!(d.pixel(1, 1), vec![0.0, 0.0, 1.0, 1.0]);
        assert_eq!(d.pixel(2, 2)[3], 0.0);

        // Encoded sampling preserves NaN payloads and signed zero, including sparse defaults.
        let fmt = PixelFormat::RGBA32F;
        let default = [f32::from_bits(0x7f80_0001), -0.0, f32::INFINITY, 1.0];
        let mut s = Surface::with_default(fmt, &default);
        let encoded: Vec<u8> = [0x7fc1_2345u32, 0x7f80_0002, 0x8000_0000, 0x3f80_0000].into_iter().flat_map(u32::to_ne_bytes).collect();
        for (x, y) in [(-301, -259), (301, 259)] {
            s.write_interleaved(Rect::new(x, y, x + 1, y + 1), &encoded);
        }
        let d = downsample(&s, 7);
        let r = Rect::new(-43, -37, 44, 38);
        let mut expected = Vec::new();
        for y in r.y0..r.y1 {
            for x in r.x0..r.x1 {
                expected.extend(s.to_interleaved(Rect::new(x * 7, y * 7, x * 7 + 1, y * 7 + 1)));
            }
        }
        assert_eq!(d.to_interleaved(r), expected);
        assert_eq!(d.to_interleaved(Rect::new(1000, 1000, 1001, 1001)), s.to_interleaved(Rect::new(1000, 1000, 1001, 1001)));
    }

    #[test]
    fn downsample_handles_negative_coordinates() {
        let mut s = Surface::new(PixelFormat::RGBA8);
        s.fill_rect(Rect::new(-8, -8, 0, 0), &[0.0, 1.0, 0.0, 1.0]);
        let d = downsample(&s, 4);
        assert_eq!(d.pixel(-1, -1), vec![0.0, 1.0, 0.0, 1.0]);
        assert_eq!(d.pixel(0, 0)[3], 0.0);
    }

    #[test]
    fn proxy_keeps_structure_and_ids() {
        let doc = Document::with_background("d", Size::new(4000, 3000), ColorMode::Rgb, SampleType::U8, Color::WHITE);
        let p = proxy_document(&doc, 4);
        assert_eq!((p.size.width, p.size.height), (1000, 750));
        assert_eq!(p.layers[0].id, doc.layers[0].id);
        assert_eq!(p.layers[0].surface().unwrap().pixel(999, 749), vec![1.0; 4]);
        assert!(proxy_faithful(&doc));
    }
}
