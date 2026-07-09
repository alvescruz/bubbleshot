pub mod shapes;

use crate::ui::types::{ROBOTO_FONT, Shape};
use ab_glyph::FontVec;
use image::RgbaImage;
use shapes::RenderCtx;
use tiny_skia::{IntSize, Pixmap};

pub fn render_to_image(
    shapes: &[Shape],
    width: u32,
    height: u32,
    image_data: &[u8],
    canvas_rect: eframe::egui::Rect,
) -> Option<RgbaImage> {
    let pixmap_data = image_data.to_vec();
    let size = IntSize::from_wh(width, height)?;
    let mut pixmap =
        Pixmap::from_vec(pixmap_data, size).unwrap_or_else(|| Pixmap::new(width, height).unwrap());

    let font = FontVec::try_from_vec(ROBOTO_FONT.to_vec()).ok();
    let ctx = RenderCtx::new(canvas_rect, width, height);

    for shape in shapes {
        use crate::ui::types::Tool;
        match shape.tool {
            Tool::Rectangle if shape.points.len() >= 2 => {
                shapes::render_rectangle(&mut pixmap, shape, &ctx);
            }
            Tool::Circle if shape.points.len() >= 2 => {
                shapes::render_circle(&mut pixmap, shape, &ctx);
            }
            Tool::Step if !shape.points.is_empty() => {
                shapes::render_step(&mut pixmap, shape, &ctx, font.as_ref());
            }
            Tool::Pen if shape.points.len() >= 2 => {
                shapes::render_pen(&mut pixmap, shape, &ctx);
            }
            Tool::Arrow if shape.points.len() >= 2 => {
                shapes::render_arrow(&mut pixmap, shape, &ctx);
            }
            Tool::Blur if shape.points.len() >= 2 => {
                shapes::render_blur(&mut pixmap, shape, &ctx);
            }
            Tool::Text if !shape.text.is_empty() && font.is_some() => {
                shapes::render_text(&mut pixmap, shape, &ctx, font.as_ref().unwrap());
            }
            _ => {}
        }
    }

    RgbaImage::from_raw(width, height, pixmap.take())
}
