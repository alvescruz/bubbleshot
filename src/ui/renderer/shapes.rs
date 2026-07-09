use crate::ui::types::{Shape, is_light_color};
use crate::ui::utils::point_to_pixel;
use ab_glyph::{Font, FontVec, OutlineCurve, PxScale, ScaleFont};
use eframe::egui;
use tiny_skia::{FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

fn make_color_paint(r: u8, g: u8, b: u8) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 255);
    paint.anti_alias = true;
    paint
}

fn make_stroke(width: f32) -> Stroke {
    Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        ..Default::default()
    }
}

fn curves_connected(a: &OutlineCurve, b: &OutlineCurve) -> bool {
    let end = match a {
        OutlineCurve::Line(_, p)
        | OutlineCurve::Quad(_, _, p)
        | OutlineCurve::Cubic(_, _, _, p) => *p,
    };
    let start = match b {
        OutlineCurve::Line(p, _)
        | OutlineCurve::Quad(p, _, _)
        | OutlineCurve::Cubic(p, _, _, _) => *p,
    };
    (end.x - start.x).abs() < 0.001 && (end.y - start.y).abs() < 0.001
}

fn build_glyph_path(curves: &[OutlineCurve], scale_x: f32, scale_y: f32) -> Option<Path> {
    let mut pb = PathBuilder::new();
    let mut i = 0;
    while i < curves.len() {
        let contour_start = i;
        while i < curves.len() {
            if i > contour_start && !curves_connected(&curves[i - 1], &curves[i]) {
                break;
            }
            match &curves[i] {
                OutlineCurve::Line(p0, p1) => {
                    if i == contour_start {
                        pb.move_to(p0.x * scale_x, -p0.y * scale_y);
                    }
                    pb.line_to(p1.x * scale_x, -p1.y * scale_y);
                }
                OutlineCurve::Quad(p0, p1, p2) => {
                    if i == contour_start {
                        pb.move_to(p0.x * scale_x, -p0.y * scale_y);
                    }
                    pb.quad_to(
                        p1.x * scale_x,
                        -p1.y * scale_y,
                        p2.x * scale_x,
                        -p2.y * scale_y,
                    );
                }
                OutlineCurve::Cubic(p0, p1, p2, p3) => {
                    if i == contour_start {
                        pb.move_to(p0.x * scale_x, -p0.y * scale_y);
                    }
                    pb.cubic_to(
                        p1.x * scale_x,
                        -p1.y * scale_y,
                        p2.x * scale_x,
                        -p2.y * scale_y,
                        p3.x * scale_x,
                        -p3.y * scale_y,
                    );
                }
            }
            i += 1;
        }
    }
    pb.finish()
}

pub struct RenderCtx {
    pub canvas_rect: egui::Rect,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

impl RenderCtx {
    pub fn new(canvas_rect: egui::Rect, width: u32, height: u32) -> Self {
        Self {
            canvas_rect,
            width,
            height,
            scale: width as f32 / canvas_rect.width(),
        }
    }
}

pub fn render_rectangle(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx) {
    let (p1, p2) = (
        point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height),
        point_to_pixel(shape.points[1], ctx.canvas_rect, ctx.width, ctx.height),
    );
    let (x1, y1) = (p1.0.min(p2.0), p1.1.min(p2.1));
    let (x2, y2) = (p1.0.max(p2.0), p1.1.max(p2.1));
    if (x2 - x1).abs() < 0.5 || (y2 - y1).abs() < 0.5 {
        return;
    }
    let mut pb = PathBuilder::new();
    pb.move_to(x1, y1);
    pb.line_to(x2, y1);
    pb.line_to(x2, y2);
    pb.line_to(x1, y2);
    pb.close();
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        let stroke = make_stroke((shape.thickness * ctx.scale).max(1.0));
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

pub fn render_circle(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx) {
    let (p1, p2) = (
        point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height),
        point_to_pixel(shape.points[1], ctx.canvas_rect, ctx.width, ctx.height),
    );
    let radius = ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt();
    if radius < 0.5 {
        return;
    }
    let mut pb = PathBuilder::new();
    pb.push_circle(p1.0, p1.1, radius);
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        let stroke = make_stroke((shape.thickness * ctx.scale).max(1.0));
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

pub fn render_step(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx, font: Option<&FontVec>) {
    let (px, py) = point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height);
    let radius = 15.0 * ctx.scale;
    let mut pb = PathBuilder::new();
    pb.push_circle(px, py, radius);
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
    if let Some(num) = shape.step_number {
        let (r, g, b) = (shape.color.r(), shape.color.g(), shape.color.b());
        let text = num.to_string();
        if let Some(font) = font {
            let px_scale = PxScale::from(radius * 1.2);
            let sf = font.as_scaled(px_scale);
            let sfac = sf.scale_factor();
            let total_w: f32 = text.chars().map(|c| sf.h_advance(sf.glyph_id(c))).sum();
            let cx = px - total_w / 2.0;
            let cy = py + f32::midpoint(sf.ascent(), sf.descent());
            let mut paint = Paint::default();
            if is_light_color(r, g, b) {
                paint.set_color_rgba8(0, 0, 0, 255);
            } else {
                paint.set_color_rgba8(255, 255, 255, 255);
            }
            paint.anti_alias = true;
            let mut cursor_x = cx;
            for c in text.chars() {
                let gid = sf.glyph_id(c);
                if let Some(outline) = font.outline(gid)
                    && let Some(path) =
                        build_glyph_path(&outline.curves, sfac.horizontal, sfac.vertical)
                {
                    pixmap.fill_path(
                        &path,
                        &paint,
                        FillRule::Winding,
                        Transform::from_translate(cursor_x, cy),
                        None,
                    );
                }
                cursor_x += sf.h_advance(gid);
            }
        }
    }
}

pub fn render_pen(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx) {
    let mut pb = PathBuilder::new();
    let (fx, fy) = point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height);
    pb.move_to(fx, fy);
    for point in &shape.points[1..] {
        let (px, py) = point_to_pixel(*point, ctx.canvas_rect, ctx.width, ctx.height);
        pb.line_to(px, py);
    }
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        let stroke = make_stroke((shape.thickness * ctx.scale).max(1.0));
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

pub fn render_arrow(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx) {
    let (start, end) = (
        point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height),
        point_to_pixel(shape.points[1], ctx.canvas_rect, ctx.width, ctx.height),
    );
    let mut pb = PathBuilder::new();
    pb.move_to(start.0, start.1);
    pb.line_to(end.0, end.1);
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        let stroke = make_stroke((shape.thickness * ctx.scale).max(1.0));
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
    let head_len = 15.0 * ctx.scale;
    let head_angle = 0.4;
    let angle = (end.1 - start.1).atan2(end.0 - start.0);
    let (lx, ly) = (
        end.0 - head_len * (angle - head_angle).cos(),
        end.1 - head_len * (angle - head_angle).sin(),
    );
    let (rx, ry) = (
        end.0 - head_len * (angle + head_angle).cos(),
        end.1 - head_len * (angle + head_angle).sin(),
    );
    let mut pb = PathBuilder::new();
    pb.move_to(end.0, end.1);
    pb.line_to(lx, ly);
    pb.line_to(rx, ry);
    pb.close();
    if let Some(path) = pb.finish() {
        let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

pub fn render_blur(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx) {
    let (p1, p2) = (
        point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height),
        point_to_pixel(shape.points[1], ctx.canvas_rect, ctx.width, ctx.height),
    );
    let (x1, y1) = (p1.0.min(p2.0), p1.1.min(p2.1));
    let (x2, y2) = (p1.0.max(p2.0), p1.1.max(p2.1));
    if (x2 - x1).abs() < 0.5 || (y2 - y1).abs() < 0.5 {
        return;
    }
    let mut pb = PathBuilder::new();
    pb.move_to(x1, y1);
    pb.line_to(x2, y1);
    pb.line_to(x2, y2);
    pb.line_to(x1, y2);
    pb.close();
    if let Some(path) = pb.finish() {
        let mut paint = Paint::default();
        paint.set_color_rgba8(0, 0, 0, 255);
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

pub fn render_text(pixmap: &mut Pixmap, shape: &Shape, ctx: &RenderCtx, font: &FontVec) {
    let (x, y) = point_to_pixel(shape.points[0], ctx.canvas_rect, ctx.width, ctx.height);
    let px_scale = PxScale::from(22.0 * ctx.scale);
    let sf = font.as_scaled(px_scale);
    let sfac = sf.scale_factor();
    let paint = make_color_paint(shape.color.r(), shape.color.g(), shape.color.b());
    let cy = y + sf.ascent();
    let mut cx = x;
    for c in shape.text.chars() {
        let gid = sf.glyph_id(c);
        if let Some(outline) = font.outline(gid)
            && let Some(path) = build_glyph_path(&outline.curves, sfac.horizontal, sfac.vertical)
        {
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::from_translate(cx, cy),
                None,
            );
        }
        cx += sf.h_advance(gid);
    }
}
