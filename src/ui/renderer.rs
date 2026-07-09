use crate::ui::types::{ROBOTO_FONT, Shape, Tool};
use crate::ui::utils::{get_arrow_points, point_to_pixel};
use ab_glyph::{FontVec, PxScale};
use eframe::egui;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect as ProcRect;

fn draw_smooth_line(
    img: &mut RgbaImage,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    thickness: f32,
    color: Rgba<u8>,
) {
    let half = thickness.max(0.5);
    let (w, h) = (img.width() as i32, img.height() as i32);
    let min_x = ((x1.min(x2) - half).floor() as i32).max(0);
    let max_x = ((x1.max(x2) + half).ceil() as i32).min(w - 1);
    let min_y = ((y1.min(y2) - half).floor() as i32).max(0);
    let max_y = ((y1.max(y2) + half).ceil() as i32).min(h - 1);

    let dx = x2 - x1;
    let dy = y2 - y1;
    let len_sq = dx * dx + dy * dy;

    let cr = color.0[0] as f32 / 255.0;
    let cg = color.0[1] as f32 / 255.0;
    let cb = color.0[2] as f32 / 255.0;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let cx = px as f32 + 0.5;
            let cy = py as f32 + 0.5;

            let dist = if len_sq < 1e-10 {
                ((cx - x1).powi(2) + (cy - y1).powi(2)).sqrt()
            } else {
                let t = ((cx - x1) * dx + (cy - y1) * dy) / len_sq;
                let t_clamped = t.clamp(0.0, 1.0);
                let proj_x = x1 + t_clamped * dx;
                let proj_y = y1 + t_clamped * dy;
                ((cx - proj_x).powi(2) + (cy - proj_y).powi(2)).sqrt()
            };

            let coverage = 1.0 - (dist / half).clamp(0.0, 1.0);
            if coverage > 0.0 {
                let old = img.get_pixel(px as u32, py as u32);
                let inv = 1.0 - coverage;
                let r = (old.0[0] as f32 * inv + cr * 255.0 * coverage) as u8;
                let g = (old.0[1] as f32 * inv + cg * 255.0 * coverage) as u8;
                let b = (old.0[2] as f32 * inv + cb * 255.0 * coverage) as u8;
                img.put_pixel(px as u32, py as u32, Rgba([r, g, b, 255]));
            }
        }
    }
}

fn draw_smooth_circle_outline(
    img: &mut RgbaImage,
    cx: f32,
    cy: f32,
    radius: f32,
    thickness: f32,
    color: Rgba<u8>,
) {
    let half = thickness.max(0.5);
    let outer_r = (radius + half).ceil() as i32;
    let min_x = (cx as i32 - outer_r).max(0);
    let max_x = (cx as i32 + outer_r).min(img.width() as i32 - 1);
    let min_y = (cy as i32 - outer_r).max(0);
    let max_y = (cy as i32 + outer_r).min(img.height() as i32 - 1);

    let cr = color.0[0] as f32 / 255.0;
    let cg = color.0[1] as f32 / 255.0;
    let cb = color.0[2] as f32 / 255.0;

    for py in min_y..=max_y {
        for px in min_x..=max_x {
            let cx_px = px as f32 + 0.5;
            let cy_px = py as f32 + 0.5;
            let d = ((cx_px - cx).powi(2) + (cy_px - cy).powi(2)).sqrt();
            let dist = (d - radius).abs();
            let coverage = 1.0 - (dist / half).clamp(0.0, 1.0);
            if coverage > 0.0 {
                let old = img.get_pixel(px as u32, py as u32);
                let inv = 1.0 - coverage;
                let r = (old.0[0] as f32 * inv + cr * 255.0 * coverage) as u8;
                let g = (old.0[1] as f32 * inv + cg * 255.0 * coverage) as u8;
                let b = (old.0[2] as f32 * inv + cb * 255.0 * coverage) as u8;
                img.put_pixel(px as u32, py as u32, Rgba([r, g, b, 255]));
            }
        }
    }
}

pub fn render_to_image(
    shapes: &[Shape],
    width: u32,
    height: u32,
    image_data: &[u8],
    canvas_rect: egui::Rect,
) -> RgbaImage {
    let mut img = RgbaImage::from_raw(width, height, image_data.to_vec())
        .unwrap_or_else(|| RgbaImage::new(width, height));
    let font = FontVec::try_from_vec(ROBOTO_FONT.to_vec()).ok();
    let scale = width as f32 / canvas_rect.width();

    for shape in shapes {
        let color = Rgba([
            shape.color.r(),
            shape.color.g(),
            shape.color.b(),
            shape.color.a(),
        ]);
        let thick = (shape.thickness * scale).max(1.0);

        match shape.tool {
            Tool::Rectangle if shape.points.len() >= 2 => {
                let (p1, p2) = (
                    point_to_pixel(shape.points[0], canvas_rect, width, height),
                    point_to_pixel(shape.points[1], canvas_rect, width, height),
                );
                let x = p1.0.min(p2.0) as i32;
                let y = p1.1.min(p2.1) as i32;
                let w = (p1.0 - p2.0).abs().round() as u32;
                let h = (p1.1 - p2.1).abs().round() as u32;
                if w > 0 && h > 0 {
                    draw_hollow_rect_mut(&mut img, ProcRect::at(x, y).of_size(w, h), color);
                }
            }
            Tool::Circle if shape.points.len() >= 2 => {
                let (p1, p2) = (
                    point_to_pixel(shape.points[0], canvas_rect, width, height),
                    point_to_pixel(shape.points[1], canvas_rect, width, height),
                );
                let radius = ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt();
                if radius > 0.5 {
                    draw_smooth_circle_outline(&mut img, p1.0, p1.1, radius, thick, color);
                }
            }
            Tool::Step if !shape.points.is_empty() => {
                render_step(&mut img, shape, &font, scale, canvas_rect, color);
            }
            Tool::Pen if shape.points.len() >= 2 => {
                let half = (thick / 2.0).round() as i32;
                for i in 0..shape.points.len().saturating_sub(1) {
                    let (p1, p2) = (
                        point_to_pixel(shape.points[i], canvas_rect, width, height),
                        point_to_pixel(shape.points[i + 1], canvas_rect, width, height),
                    );
                    draw_smooth_line(&mut img, p1.0, p1.1, p2.0, p2.1, thick, color);
                }
                if half > 0 {
                    for point in &shape.points {
                        let (px, py) = point_to_pixel(*point, canvas_rect, width, height);
                        draw_filled_circle_mut(
                            &mut img,
                            (px as i32, py as i32),
                            half.max(1),
                            color,
                        );
                    }
                }
            }
            Tool::Arrow if shape.points.len() >= 2 => {
                let (p1, p2) = (
                    point_to_pixel(shape.points[0], canvas_rect, width, height),
                    point_to_pixel(shape.points[1], canvas_rect, width, height),
                );
                draw_smooth_line(&mut img, p1.0, p1.1, p2.0, p2.1, thick, color);
                let pts = get_arrow_points(shape.points[0], shape.points[1]);
                for i in (2..pts.len()).step_by(2) {
                    let (a, b) = (
                        point_to_pixel(pts[i], canvas_rect, width, height),
                        point_to_pixel(pts[i + 1], canvas_rect, width, height),
                    );
                    draw_smooth_line(&mut img, a.0, a.1, b.0, b.1, thick, color);
                }
            }
            Tool::Blur if shape.points.len() >= 2 => {
                let (p1, p2) = (
                    point_to_pixel(shape.points[0], canvas_rect, width, height),
                    point_to_pixel(shape.points[1], canvas_rect, width, height),
                );
                let (rx, ry, rw, rh) = (
                    p1.0.min(p2.0) as u32,
                    p1.1.min(p2.1) as u32,
                    (p1.0 - p2.0).abs() as u32,
                    (p1.1 - p2.1).abs() as u32,
                );
                if rw > 0 && rh > 0 {
                    let black = Rgba([0, 0, 0, 255]);
                    for y in ry..(ry + rh) {
                        for x in rx..(rx + rw) {
                            if x < width && y < height {
                                img.put_pixel(x, y, black);
                            }
                        }
                    }
                }
            }
            Tool::Text if !shape.text.is_empty() => {
                if let Some(ref f) = font {
                    let (x, y) = point_to_pixel(shape.points[0], canvas_rect, width, height);
                    draw_text_mut(
                        &mut img,
                        color,
                        x as i32,
                        y as i32,
                        PxScale::from(22.0 * scale),
                        f,
                        &shape.text,
                    );
                }
            }
            _ => {}
        }
    }
    img
}

fn render_step(
    img: &mut RgbaImage,
    shape: &Shape,
    font: &Option<FontVec>,
    scale: f32,
    canvas_rect: egui::Rect,
    color: Rgba<u8>,
) {
    let img_w = img.width();
    let img_h = img.height();
    let (px, py) = point_to_pixel(shape.points[0], canvas_rect, img_w, img_h);
    let base_radius = 15.0;
    let scaled_radius = (base_radius * scale).round() as i32;

    draw_filled_circle_mut(img, (px as i32, py as i32), scaled_radius.max(1), color);
    if let (Some(f), Some(num)) = (font, shape.step_number) {
        let text_color =
            if shape.color.r() as u32 + shape.color.g() as u32 + shape.color.b() as u32 > 382 {
                Rgba([0, 0, 0, 255])
            } else {
                Rgba([255, 255, 255, 255])
            };
        let text = num.to_string();
        let text_scale = scaled_radius as f32 * 1.2;
        let approx_w = text_scale * 0.65 * (text.len() as f32);
        let tx = (px - approx_w / 2.0) as i32;
        let ty = (py - text_scale * 0.4) as i32;
        draw_text_mut(img, text_color, tx, ty, PxScale::from(text_scale), f, &text);
    }
}
