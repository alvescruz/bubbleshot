use crate::ui::components::{action_button, styled_button};
use crate::ui::painter::draw_shape;
use crate::ui::renderer::render_to_image;
use crate::ui::types::{ROBOTO_FONT, Shape, Tool};
use crate::ui::utils::get_canvas_rect;
use arboard::{Clipboard, ImageData};
use eframe::egui::{self};
use egui_phosphor::regular as icons;
use rfd::FileDialog;
use std::borrow::Cow;

pub struct SelectionApp {
    pub image_data: Option<Vec<u8>>,
    pub width: u32,
    pub height: u32,

    texture: Option<egui::TextureHandle>,
    current_tool: Tool,
    shapes: Vec<Shape>,
    redo_stack: Vec<Shape>,
    current_shape: Option<Shape>,

    stroke_color: egui::Color32,
    initialized_fonts: bool,

    selected_shape_index: Option<usize>,
    hover_shape_index: Option<usize>,
    current_step: usize,
    show_exit_confirmation: bool,
}

impl SelectionApp {
    pub fn new(image_data: Option<Vec<u8>>, width: u32, height: u32) -> Self {
        Self {
            image_data,
            width,
            height,
            texture: None,
            current_tool: Tool::Pen,
            shapes: Vec::new(),
            redo_stack: Vec::new(),
            current_shape: None,
            stroke_color: egui::Color32::from_rgb(255, 0, 0),
            initialized_fonts: false,
            selected_shape_index: None,
            hover_shape_index: None,
            current_step: 1,
            show_exit_confirmation: false,
        }
    }

    fn finish_text_if_any(&mut self) {
        if let Some(shape) = self.current_shape.take() {
            if shape.tool == Tool::Text && !shape.text.is_empty() {
                self.shapes.push(shape);
                self.redo_stack.clear();
            } else if shape.tool != Tool::Text {
                // If for some reason there's another pending shape, put it back
                self.current_shape = Some(shape);
            }
        }
    }

    fn setup_fonts(&mut self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        // Register Roboto-Regular
        fonts.font_data.insert(
            "roboto".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(ROBOTO_FONT)),
        );

        // Set Roboto as default for proportional text
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "roboto".to_owned());

        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
        self.initialized_fonts = true;
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context, canvas_rect: egui::Rect) {
        if self.show_exit_confirmation {
            return;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_exit_confirmation = true;
        }

        // 1. Collect actions within the closure (without touching self)
        let (save, copy, redo, undo) = ctx.input_mut(|input| {
            let save = input.consume_key(egui::Modifiers::COMMAND, egui::Key::S);

            // Copy: Ctrl+C or native Copy event
            let copy = input.consume_key(egui::Modifiers::COMMAND, egui::Key::C)
                || input.events.iter().any(|e| matches!(e, egui::Event::Copy));

            // Redo: Ctrl+Shift+Z or Ctrl+Y
            let redo = input.consume_key(
                egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
                egui::Key::Z,
            ) || input.consume_key(egui::Modifiers::COMMAND, egui::Key::Y);

            // Undo: Ctrl+Z
            let undo = input.consume_key(egui::Modifiers::COMMAND, egui::Key::Z);

            (save, copy, redo, undo)
        });

        // 2. Execute actions outside the closure (self is available)
        if save {
            self.finish_text_if_any();
            self.save_action(canvas_rect);
        }

        if copy {
            self.finish_text_if_any();
            self.copy_action(ctx, canvas_rect);
        }

        // Redo before Undo to avoid conflict in key consumption
        if redo {
            if let Some(s) = self.redo_stack.pop() {
                self.shapes.push(s);
            }
        } else if undo && let Some(s) = self.shapes.pop() {
            self.redo_stack.push(s);
        }
    }

    fn save_action(&self, canvas_rect: egui::Rect) {
        if let Some(data) = &self.image_data {
            let img = render_to_image(&self.shapes, self.width, self.height, data, canvas_rect);
            if let Some(path) = FileDialog::new().set_file_name("shot.png").save_file() {
                if let Err(e) = img.save(path) {
                    eprintln!("[theoshot] Error saving image: {}", e);
                } else {
                    std::process::exit(0);
                }
            }
        }
    }

    fn copy_action(&self, ctx: &egui::Context, canvas_rect: egui::Rect) {
        if let Some(data) = &self.image_data {
            // Hide the window immediately to provide feedback
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));

            let shapes = self.shapes.clone();
            let width = self.width;
            let height = self.height;
            let data = data.clone();

            std::thread::spawn(move || {
                let img = render_to_image(&shapes, width, height, &data, canvas_rect);
                match Clipboard::new() {
                    Ok(mut cb) => {
                        let (w, h) = img.dimensions();
                        let raw_bytes = img.into_raw();
                        let image_data = ImageData {
                            width: w as usize,
                            height: h as usize,
                            bytes: Cow::from(raw_bytes),
                        };
                        if let Err(e) = cb.set_image(image_data) {
                            eprintln!("[theoshot] Error copying image: {}", e);
                        } else {
                            // Reduced delay for Linux persistence
                            std::thread::sleep(std::time::Duration::from_millis(150));
                        }
                    }
                    Err(e) => {
                        eprintln!("[theoshot] Error accessing clipboard: {}", e);
                    }
                }
                std::process::exit(0);
            });
        }
    }

    fn load_background_texture(&mut self, ctx: &egui::Context) {
        if self.texture.is_none()
            && let Some(data) = &self.image_data
        {
            let size = [self.width as usize, self.height as usize];

            self.texture = Some(ctx.load_texture(
                "bg",
                egui::ColorImage::from_rgba_unmultiplied(size, data),
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Linear,
                    ..Default::default()
                },
            ));
        }
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui, canvas_rect: egui::Rect) {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let toolbar_width = 560.0;
            ui.add_space((canvas_rect.width() - toolbar_width).max(0.0) / 2.0);

            egui::Frame::default()
                .fill(egui::Color32::from_rgb(40, 40, 45))
                .rounding(egui::Rounding::same(14.0))
                .inner_margin(10.0)
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(35)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(10.0, 0.0);

                        // Color selector
                        ui.vertical(|ui| {
                            ui.add_space(7.0); // Manual vertical centering
                            ui.scope(|ui| {
                                ui.visuals_mut().widgets.inactive.rounding =
                                    egui::Rounding::same(11.0);
                                ui.visuals_mut().widgets.hovered.rounding =
                                    egui::Rounding::same(11.0);
                                ui.visuals_mut().widgets.active.rounding =
                                    egui::Rounding::same(11.0);
                                ui.spacing_mut().interact_size = egui::vec2(22.0, 22.0);
                                ui.color_edit_button_srgba(&mut self.stroke_color);
                            });
                        });

                        ui.separator();

                        let tools = [
                            (Tool::Rectangle, icons::RECTANGLE, "Rectangle"),
                            (Tool::Circle, icons::CIRCLE, "Circle"),
                            (Tool::Step, icons::LIST_NUMBERS, "Steps"),
                            (Tool::Pen, icons::PENCIL_SIMPLE, "Pen"),
                            (Tool::Arrow, icons::ARROW_UP_RIGHT, "Arrow"),
                            (Tool::Blur, icons::DROP, "Blur"),
                            (Tool::Text, icons::TEXT_T, "Text"),
                            (Tool::Move, icons::CURSOR_CLICK, "Move"),
                        ];

                        for (t, icon, name) in tools {
                            let is_active = self.current_tool == t;
                            let icon_color = if is_active {
                                if self.stroke_color.r() as u32
                                    + self.stroke_color.g() as u32
                                    + self.stroke_color.b() as u32
                                    > 382
                                {
                                    egui::Color32::BLACK
                                } else {
                                    egui::Color32::WHITE
                                }
                            } else {
                                egui::Color32::WHITE
                            };

                            if styled_button(
                                ui,
                                egui::RichText::new(icon).size(20.0).color(icon_color),
                                is_active,
                                self.stroke_color,
                            )
                            .on_hover_text(name)
                            .clicked()
                            {
                                if self.current_tool == Tool::Text && t != Tool::Text {
                                    self.finish_text_if_any();
                                }
                                self.current_tool = t;
                                if t != Tool::Move {
                                    self.selected_shape_index = None;
                                }
                            }
                        }

                        ui.separator();
                        if styled_button(
                            ui,
                            egui::RichText::new(icons::ARROW_U_UP_LEFT).size(18.0),
                            false,
                            self.stroke_color,
                        )
                        .on_hover_text("Undo (Ctrl+Z)")
                        .clicked()
                        {
                            self.finish_text_if_any();
                            if let Some(s) = self.shapes.pop() {
                                self.redo_stack.push(s);
                            }
                        }
                        if styled_button(
                            ui,
                            egui::RichText::new(icons::ARROW_U_UP_RIGHT).size(18.0),
                            false,
                            self.stroke_color,
                        )
                        .on_hover_text("Redo (Ctrl+Shift+Z)")
                        .clicked()
                        {
                            self.finish_text_if_any();
                            if let Some(s) = self.redo_stack.pop() {
                                self.shapes.push(s);
                            }
                        }

                        if styled_button(
                            ui,
                            egui::RichText::new(icons::TRASH).size(18.0),
                            false,
                            self.stroke_color,
                        )
                        .on_hover_text("Clear All")
                        .clicked()
                        {
                            self.finish_text_if_any();
                            self.shapes.clear();
                            self.redo_stack.clear();
                            self.current_step = 1;
                            self.selected_shape_index = None;
                        }

                        ui.separator();
                        if styled_button(
                            ui,
                            egui::RichText::new(icons::COPY_SIMPLE).size(20.0),
                            false,
                            self.stroke_color,
                        )
                        .on_hover_text("Copy (Ctrl+C)")
                        .clicked()
                        {
                            self.finish_text_if_any();
                            self.copy_action(ui.ctx(), canvas_rect);
                        }

                        if action_button(
                            ui,
                            egui::RichText::new(icons::FLOPPY_DISK)
                                .size(20.0)
                                .color(egui::Color32::WHITE),
                            egui::Color32::from_rgb(60, 130, 255),
                        )
                        .on_hover_text("Save (Ctrl+S)")
                        .clicked()
                        {
                            self.finish_text_if_any();
                            self.save_action(canvas_rect);
                        }
                    });
                });
        });
    }

    fn handle_canvas_interactions(&mut self, ctx: &egui::Context, res: egui::Response) {
        if self.show_exit_confirmation {
            self.hover_shape_index = None;
            return;
        }
        let pos = res.interact_pointer_pos();

        // Hover and Cursor
        if self.current_tool == Tool::Move {
            if let Some(p) = pos {
                let mut nearest = None;
                let mut min_dist = 20.0;
                for (i, shape) in self.shapes.iter().enumerate() {
                    let bbox = shape.bounding_box();

                    // For shapes with a "body" (Rectangle, Circle, Blur, Text, Step),
                    // we check if the mouse is inside the area.
                    let is_inside = match shape.tool {
                        Tool::Rectangle | Tool::Blur | Tool::Text | Tool::Step => bbox.contains(p),
                        Tool::Circle if shape.points.len() >= 2 => {
                            let center = shape.points[0];
                            let radius = center.distance(shape.points[1]);
                            p.distance(center) <= radius
                        }
                        _ => false,
                    };

                    if is_inside {
                        nearest = Some(i);
                        break; // Priority for what is on top
                    }

                    // If not inside, or for line tools (Pen, Arrow),
                    // we check point proximity.
                    if bbox.expand(min_dist).contains(p) {
                        for pt in &shape.points {
                            let dist = p.distance(*pt);
                            if dist < min_dist {
                                min_dist = dist;
                                nearest = Some(i);
                            }
                        }
                    }
                }
                self.hover_shape_index = nearest;
                if self.hover_shape_index.is_some() {
                    ctx.set_cursor_icon(egui::CursorIcon::Grab);
                }
            } else {
                self.hover_shape_index = None;
            }
            if res.dragged() && self.selected_shape_index.is_some() {
                ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            }
        }

        // Drag Start
        if res.drag_started() {
            self.finish_text_if_any();
            if let Some(p) = pos {
                if self.current_tool == Tool::Move {
                    self.selected_shape_index = self.hover_shape_index;
                } else {
                    self.current_shape = Some(Shape {
                        tool: self.current_tool,
                        points: vec![p, p],
                        color: self.stroke_color,
                        thickness: 1.5,
                        text: String::new(),
                        step_number: if self.current_tool == Tool::Step {
                            Some(self.current_step)
                        } else {
                            None
                        },
                    });
                }
            }
        }

        // Dragging
        if res.dragged()
            && let Some(p) = pos
        {
            if self.current_tool == Tool::Move {
                if let Some(idx) = self.selected_shape_index {
                    let delta = res.drag_delta();
                    if let Some(shape) = self.shapes.get_mut(idx) {
                        for pt in &mut shape.points {
                            *pt += delta;
                        }
                    }
                }
            } else if let Some(shape) = &mut self.current_shape {
                if shape.tool == Tool::Pen {
                    shape.points.push(p);
                } else {
                    shape.points[1] = p;
                }
            }
        }

        // Drag Stop
        if res.drag_stopped()
            && let Some(shape) = self.current_shape.take()
        {
            if shape.tool != Tool::Text {
                if shape.tool == Tool::Step {
                    self.current_step += 1;
                }
                self.shapes.push(shape);
                self.redo_stack.clear();
            } else {
                self.current_shape = Some(shape);
            }
        }

        // Text input handling
        if self.current_tool == Tool::Text
            && let Some(shape) = &mut self.current_shape
        {
            let mut finished = false;
            let events = ctx.input(|i| i.events.clone());
            for event in events {
                match event {
                    egui::Event::Text(t) => shape.text.push_str(&t),
                    egui::Event::Key {
                        key: egui::Key::Backspace,
                        pressed: true,
                        ..
                    } => {
                        shape.text.pop();
                    }
                    egui::Event::Key {
                        key: egui::Key::Enter,
                        pressed: true,
                        ..
                    } => {
                        finished = true;
                    }
                    _ => {}
                }
            }
            if finished {
                self.finish_text_if_any();
            }
        }
    }

    fn setup_visuals(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::TRANSPARENT;
        visuals.window_fill = egui::Color32::TRANSPARENT;
        ctx.set_visuals(visuals);
    }
}

impl eframe::App for SelectionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized_fonts {
            self.setup_fonts(ctx);
            self.setup_visuals(ctx);
        }

        let canvas_rect = get_canvas_rect(ctx, self.width, self.height);
        self.load_background_texture(ctx);
        self.handle_shortcuts(ctx, canvas_rect);

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |_ui| {
                egui::Area::new(egui::Id::new("main_view"))
                    .fixed_pos(canvas_rect.min)
                    .show(ctx, |ui| {
                        egui::Frame::default()
                            .fill(egui::Color32::from_rgb(30, 30, 35))
                            .rounding(egui::Rounding::ZERO)
                            .shadow(egui::Shadow {
                                blur: 40.0,
                                offset: egui::vec2(0.0, 20.0),
                                color: egui::Color32::from_black_alpha(200),
                                ..Default::default()
                            })
                            .stroke(egui::Stroke::new(1.5, egui::Color32::from_white_alpha(45)))
                            .show(ui, |ui| {
                                ui.set_width(canvas_rect.width());
                                let (res, painter) =
                                    ui.allocate_painter(canvas_rect.size(), egui::Sense::drag());

                                if let Some(tex) = &self.texture {
                                    painter.image(
                                        tex.id(),
                                        canvas_rect,
                                        egui::Rect::from_min_max(
                                            egui::pos2(0.0, 0.0),
                                            egui::pos2(1.0, 1.0),
                                        ),
                                        egui::Color32::WHITE,
                                    );
                                }

                                for (i, shape) in self.shapes.iter().enumerate() {
                                    draw_shape(
                                        &painter,
                                        shape,
                                        false,
                                        ctx,
                                        Some(i) == self.selected_shape_index,
                                        Some(i) == self.hover_shape_index,
                                    );
                                }
                                if let Some(shape) = &self.current_shape {
                                    draw_shape(&painter, shape, true, ctx, false, false);
                                }

                                self.handle_canvas_interactions(ctx, res);
                            });

                        self.render_toolbar(ui, canvas_rect);
                    });
            });

        self.render_exit_confirmation_dialog(ctx);

        ctx.request_repaint();
    }
}

impl SelectionApp {
    fn render_exit_confirmation_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_exit_confirmation {
            return;
        }

        egui::Window::new("Confirm Exit")
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .frame(
                egui::Frame::window(&ctx.style())
                    .fill(egui::Color32::from_rgb(35, 35, 40))
                    .rounding(10.0)
                    .shadow(egui::Shadow {
                        blur: 20.0,
                        offset: egui::vec2(0.0, 8.0),
                        color: egui::Color32::from_black_alpha(150),
                        ..Default::default()
                    })
                    .inner_margin(egui::Margin {
                        left: 12.0,
                        right: 12.0,
                        top: 16.0,
                        bottom: 16.0,
                    })
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_white_alpha(30))),
            )
            .show(ctx, |ui| {
                ui.set_max_width(220.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Really want to exit?")
                            .size(17.0)
                            .strong()
                            .color(egui::Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Changes will be lost.")
                            .size(12.0)
                            .color(egui::Color32::from_gray(160)),
                    );
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        let button_width = 85.0;
                        let button_height = 26.0;
                        ui.add_space((ui.available_width() - (button_width * 2.0 + 8.0)) / 2.0);

                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new("Cancel")
                                    .fill(egui::Color32::from_gray(60))
                                    .rounding(8.0),
                            )
                            .clicked()
                        {
                            self.show_exit_confirmation = false;
                        }

                        ui.add_space(8.0);

                        if ui
                            .add_sized(
                                [button_width, button_height],
                                egui::Button::new(
                                    egui::RichText::new("Exit").color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(180, 45, 45))
                                .rounding(8.0),
                            )
                            .clicked()
                        {
                            std::process::exit(0);
                        }
                    });
                });
            });
    }
}
