use std::cmp::Ordering;

use eframe::{
    egui::{
        self, pos2, vec2, CollapsingHeader, Frame, Painter, Pos2, Slider, Stroke, Ui, Vec2, Visuals,
    },
    epi::{self, App},
};

use crate::segment::Segment;

#[derive(PartialEq)]
pub struct KinematicsApp {
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
        Self {
            paused: false,
            length: 10.0,
            n_segments: 50,
            width_factor: 0.0,
            width: 1.0,
            segments: vec![],
            regenerate: true,
            prev_pos: Default::default(),
        }
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

    /// Paints all segments. Segments amount must be greater than 1.
    fn paint(&mut self, painter: &Painter, cursor_pos: Pos2) {
        let mut shapes = Vec::with_capacity(self.n_segments);

        self.segments[0].follow(cursor_pos);
        self.segments[0].update();
        shapes.push(self.segments[0].show());

        for n in 1..self.segments.len() {
            let pos2 = self.segments[n - 1].start;
            let segment = &mut self.segments[n];
            segment.follow(pos2);
            segment.update();
            shapes.push(segment.show());
        }

        painter.extend(shapes);
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.paused, "Paused");

        let n_segments = ui.add(Slider::new(&mut self.n_segments, 2..=500).text("segments number"));
        let length = ui.add(Slider::new(&mut self.length, 0.1..=100.0).text("length"));
        let width = ui.add(Slider::new(&mut self.width, 0.1..=100.0).text("width"));
        let width_factor =
            ui.add(Slider::new(&mut self.width_factor, -5.0..=5.0).text("width factor"));

        if [n_segments, length, width, width_factor]
            .iter()
            .any(|x| x.changed())
        {
            self.regenerate = true;
        }
        egui::reset_button(ui, self);
    }

    fn regenerate(&mut self) {
        let prev_n_segments = self.segments.len();

        if let Some(segment) = self.segments.first_mut() {
            segment.length = self.length;
            segment.width = self.width;
            segment.update();
        }

        for n in 1..prev_n_segments {
            let pos2 = self.segments[n - 1].start;
            let segment = &mut self.segments[n];
            segment.length = self.length;
            segment.width = self.width + (self.width_factor * n as f32);
            segment.follow(pos2);
            segment.update();
        }

        match prev_n_segments.cmp(&self.n_segments) {
            Ordering::Less => {
                self.segments
                    .extend((prev_n_segments..self.n_segments).map(|n| {
                        Segment::new(
                            pos2(self.length * n as f32, 0.0),
                            self.length,
                            self.width + (self.width_factor * n as f32),
                        )
                    }));
            }
            Ordering::Greater => {
                self.segments.truncate(self.n_segments);
            }
            Ordering::Equal => {}
        }

        self.regenerate = false;
    }
}

impl App for KinematicsApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(Visuals::dark());
            let cursor_pos = ctx.input().pointer.interact_pos();
            if let Some(pos) = cursor_pos {
                self.main_ui(ui, pos);
            } else {
                // go towards left lower corner until it all disappear
                let add = if self
                    .segments
                    .last()
                    .map(|segment| segment.end.x > 0.0)
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

    fn max_size_points(&self) -> Vec2 {
        // Fullscreen
        vec2(f32::MAX, f32::MAX)
    }
}
