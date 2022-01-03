use eframe::{
    egui::{
        self, pos2, vec2, Button, CollapsingHeader, Frame, Painter, Pos2, Slider, Stroke, Ui, Vec2,
    },
    epi::{self, App},
};

use segment::Segment;

mod segment;

#[derive(PartialEq)]
struct KinematicsApp {
    paused: bool,
    length: f32,
    n_segments: usize,
    width_factor: f32,
    width: f32,
    segments: Vec<Segment>,
    regenerate: bool,
    prev_pos: Pos2,
}

impl Default for KinematicsApp {
    fn default() -> Self {
        let mut app = Self {
            paused: false,
            length: 10.0,
            n_segments: 50,
            width_factor: 0.0,
            width: 1.0,
            segments: vec![],
            regenerate: false,
            prev_pos: Default::default(),
        };
        app.regenerate();
        app
    }
}

impl KinematicsApp {
    pub fn main_ui(&mut self, ui: &mut Ui, target_pos: Pos2) {
        ui.ctx().request_repaint();
        if self.regenerate {
            self.regenerate()
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );

        if self.paused {
            self.paint(&painter, self.prev_pos);
        } else {
            self.paint(&painter, target_pos);
            self.prev_pos = target_pos;
        };

        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                CollapsingHeader::new("Settings").show(ui, |ui| self.options_ui(ui));
            });
    }

    fn paint(&mut self, painter: &Painter, cursor_pos: Pos2) {
        let mut shapes = Vec::with_capacity(self.n_segments);

        self.segments[0].follow(cursor_pos);
        self.segments[0].update();
        shapes.push(self.segments[0].show());

        for i in 1..self.segments.len() {
            let pos2 = self.segments[i - 1].start();
            let segment = &mut self.segments[i];
            segment.follow(pos2);
            segment.update();
            shapes.push(segment.show());
        }

        painter.extend(shapes);
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.paused, "Paused");
        ui.add(Slider::new(&mut self.n_segments, 0..=500).text("segments number"));
        ui.add(Slider::new(&mut self.length, 0.0..=100.0).text("length"));
        ui.add(Slider::new(&mut self.width, 0.0..=100.0).text("width"));
        ui.add(Slider::new(&mut self.width_factor, -5.0..=5.0).text("width factor"));
        if ui.add(Button::new("Regenerate")).clicked() {
            self.regenerate = true;
        }
        egui::reset_button(ui, self);
    }

    fn regenerate(&mut self) {
        self.segments = (0..self.n_segments)
            .map(|n| {
                Segment::new(
                    pos2(0.0 + self.length * n as f32, 0.0),
                    self.length,
                    self.width + (self.width_factor * n as f32),
                )
            })
            .collect();
        self.regenerate = false;
    }
}

impl App for KinematicsApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let cursor_pos = ctx.input().pointer.interact_pos();
            if let Some(pos) = cursor_pos {
                self.main_ui(ui, pos);
            } else {
                // go towards left lower corner until it all disappear
                let add = if self
                    .segments
                    .last()
                    .map(|segment| segment.end().x > 0.0)
                    .unwrap_or(true)
                {
                    vec2(-1.0, 1.0)
                } else {
                    Vec2::ZERO
                };
                self.main_ui(ui, self.prev_pos + add);
            }
        });
    }

    fn name(&self) -> &str {
        "Inverse kinematics"
    }
}

fn main() {
    let app = KinematicsApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
