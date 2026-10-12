//! A CPU raster of the harness's document: the whole viewport, or a pixel rectangle of it.

use anyrender::render_to_buffer;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use blitz_dom::Document;
use blitz_paint::paint_scene;

use crate::Harness;

/// A rectangle of device pixels, its origin at the viewport's top-left.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PixelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// What a screenshot holds: `width` × `height` pixels, row by row, four bytes (RGBA8) each.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Why a screenshot was not taken. Nothing is painted in either case.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScreenshotError {
    /// The document's font context resolves no family to paint text with
    /// ([`blitz_dom::BaseDocument::can_paint_text`]).
    FontsUnavailable,
    /// The rectangle asked for holds no pixel of the viewport.
    EmptyRect,
}

impl<D: Document> Harness<D> {
    /// The whole viewport as painted now, at the harness's own time: the document painted at
    /// the viewport's scale and device size, over opaque white. Does not tick or pump.
    pub fn screenshot(&mut self) -> Result<Raster, ScreenshotError> {
        self.paint_viewport()
    }

    /// The pixels of `rect` in [`screenshot`](Self::screenshot)'s frame. A rectangle reaching
    /// past the viewport is clipped to it; one that holds no pixel of it is an error.
    pub fn screenshot_rect(&mut self, rect: PixelRect) -> Result<Raster, ScreenshotError> {
        let frame = self.paint_viewport()?;
        let x0 = rect.x.min(frame.width);
        let y0 = rect.y.min(frame.height);
        let x1 = rect.x.saturating_add(rect.width).min(frame.width);
        let y1 = rect.y.saturating_add(rect.height).min(frame.height);
        if x1 <= x0 || y1 <= y0 {
            return Err(ScreenshotError::EmptyRect);
        }
        let row_bytes = frame.width as usize * 4;
        let (left, right) = (x0 as usize * 4, x1 as usize * 4);
        let rgba = frame
            .rgba
            .chunks_exact(row_bytes)
            .skip(y0 as usize)
            .take((y1 - y0) as usize)
            .filter_map(|row| row.get(left..right))
            .flatten()
            .copied()
            .collect();
        Ok(Raster {
            width: x1 - x0,
            height: y1 - y0,
            rgba,
        })
    }

    fn paint_viewport(&mut self) -> Result<Raster, ScreenshotError> {
        let time = self.time();
        let mut doc = self.base_mut();
        if !doc.can_paint_text() {
            return Err(ScreenshotError::FontsUnavailable);
        }
        doc.resolve(time);
        let viewport = doc.get_viewport();
        let (width, height) = viewport.window_size;
        let scale = viewport.scale_f64();
        let mut rgba = render_to_buffer::<VelloCpuImageRenderer, _>(
            |scene| paint_scene(scene, &mut doc, scale, width, height, 0, 0),
            width,
            height,
        );
        // The renderer writes premultiplied RGBA8, so source-over white adds the uncovered
        // part, 255 - alpha, to each channel.
        for [red, green, blue, alpha] in rgba.as_chunks_mut::<4>().0 {
            let uncovered = 255 - *alpha;
            for channel in [red, green, blue] {
                *channel = channel.saturating_add(uncovered);
            }
            *alpha = 255;
        }
        Ok(Raster {
            width,
            height,
            rgba,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HarnessOptions;

    const PAGE: &str = r#"<html><body style="margin:0; background:#ff0000;">
        <div style="position:absolute; left:10px; top:20px; width:30px; height:40px; background:#0000ff;"></div>
    </body></html>"#;

    fn harness(font_ctx: Option<blitz_dom::FontContext>) -> Harness {
        Harness::from_html_with(
            PAGE,
            HarnessOptions {
                width: 100,
                height: 80,
                font_ctx,
                ..Default::default()
            },
        )
    }

    fn bullet_font() -> Option<blitz_dom::FontContext> {
        Some(blitz_dom::build_single_font_ctx(blitz_dom::BULLET_FONT))
    }

    fn pixel(raster: &Raster, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * raster.width + x) * 4) as usize;
        [
            raster.rgba[i],
            raster.rgba[i + 1],
            raster.rgba[i + 2],
            raster.rgba[i + 3],
        ]
    }

    #[test]
    fn the_whole_viewport_is_painted_at_its_device_size() {
        let shot = harness(bullet_font()).screenshot().unwrap();
        assert_eq!((shot.width, shot.height), (100, 80));
        assert_eq!(shot.rgba.len(), 100 * 80 * 4);
        assert_eq!(pixel(&shot, 0, 0), [255, 0, 0, 255]);
        assert_eq!(pixel(&shot, 20, 30), [0, 0, 255, 255]);
    }

    #[test]
    fn what_the_page_leaves_unpainted_reads_opaque_white() {
        let mut harness = Harness::from_html_with(
            r#"<html><body style="margin:0;">
                <div style="width:20px; height:20px; background:rgba(0,0,255,0.5);"></div>
            </body></html>"#,
            HarnessOptions {
                width: 100,
                height: 80,
                font_ctx: bullet_font(),
                ..Default::default()
            },
        );
        let shot = harness.screenshot().unwrap();
        assert_eq!(pixel(&shot, 50, 50), [255, 255, 255, 255]);
        let [r, g, b, a] = pixel(&shot, 5, 5);
        assert_eq!((b, a), (255, 255));
        assert!(
            (126..=129).contains(&r) && r == g,
            "half blue over white: {r} {g}"
        );
    }

    #[test]
    fn a_rectangle_is_that_part_of_the_frame() {
        let mut harness = harness(bullet_font());
        let whole = harness.screenshot().unwrap();
        let rect = PixelRect {
            x: 5,
            y: 15,
            width: 20,
            height: 10,
        };
        let part = harness.screenshot_rect(rect).unwrap();
        assert_eq!((part.width, part.height), (20, 10));
        for y in 0..10 {
            for x in 0..20 {
                assert_eq!(pixel(&part, x, y), pixel(&whole, x + 5, y + 15));
            }
        }
    }

    #[test]
    fn a_rectangle_past_the_viewport_is_clipped_and_one_outside_it_is_an_error() {
        let mut harness = harness(bullet_font());
        let clipped = PixelRect {
            x: 90,
            y: 70,
            width: 50,
            height: 50,
        };
        let part = harness.screenshot_rect(clipped).unwrap();
        assert_eq!((part.width, part.height), (10, 10));
        let outside = PixelRect {
            x: 100,
            y: 0,
            width: 5,
            height: 5,
        };
        assert_eq!(
            harness.screenshot_rect(outside),
            Err(ScreenshotError::EmptyRect)
        );
        let empty = PixelRect {
            x: 10,
            y: 10,
            width: 0,
            height: 5,
        };
        assert_eq!(
            harness.screenshot_rect(empty),
            Err(ScreenshotError::EmptyRect)
        );
    }

    #[test]
    fn a_document_with_no_font_is_refused() {
        let mut harness = harness(Some(blitz_dom::build_single_font_ctx(&[0u8; 16])));
        assert_eq!(harness.screenshot(), Err(ScreenshotError::FontsUnavailable));
        let rect = PixelRect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        assert_eq!(
            harness.screenshot_rect(rect),
            Err(ScreenshotError::FontsUnavailable)
        );
    }

    #[test]
    fn a_shot_is_the_same_twice() {
        let mut harness = harness(bullet_font());
        assert_eq!(harness.screenshot(), harness.screenshot());
    }
}
