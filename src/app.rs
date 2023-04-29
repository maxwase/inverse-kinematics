use std::cmp::Ordering;

use eframe::{
    egui::{
        self, pos2, vec2, CollapsingHeader, Frame as EguiFrame, Painter, Pos2, Slider, Ui, Vec2,
        Visuals,
    },
    App, Frame,
};

use crate::segment::Segment;

#[derive(PartialEq)]
pub struct KinematicsApp {
    /// Actual "snake" made of [Segment]s.
    segments: Vec<Segment>,

    /// Each segment length.
    length: f32,
    /// Requested segments.
    segments_amount: usize,
    /// Each segment width.
    width: f32,
    width_factor: f32,

    /// Current direction.
    current_target_pos: Pos2,
    /// Stop following.
    paused: bool,
    /// Shows that parameters have changed.
    regenerate: bool,
}

impl Default for KinematicsApp {
    fn default() -> Self {
        Self {
            segments: vec![],
            length: 10.0,
            segments_amount: 50,
            width: 1.0,
            width_factor: 0.0,
            current_target_pos: Pos2::default(),
            paused: false,
            regenerate: true,
        }
    }
}

impl KinematicsApp {
    /// Adds all UI elements.
    fn main_ui(&mut self, ui: &mut Ui, target_pos: Pos2) {
        if self.regenerate {
            self.regenerate(target_pos);
        }

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap(),
        );

        if !self.paused {
            self.current_target_pos = target_pos;
        };
        self.paint(&painter, self.current_target_pos);

        // Make sure we allocate what we used (everything)
        ui.expand_to_include_rect(painter.clip_rect());

        EguiFrame::popup(ui.style()).show(ui, |ui| {
            CollapsingHeader::new("Settings").show(ui, |ui| self.add_ui_options(ui));
        });
    }

    /// Paints all segments.
    fn paint(&mut self, painter: &Painter, cursor_pos: Pos2) {
        self.segments
            .iter_mut()
            .fold(cursor_pos, |target, segment| {
                segment.follow(target);
                painter.add(segment.show());
                // Remember, it is inverse!
                segment.start()
            });
    }

    /// Adds UI options for the app.
    fn add_ui_options(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.paused, "Paused");

        let n_segments =
            ui.add(Slider::new(&mut self.segments_amount, 1..=500).text("segments number"));
        let length = ui.add(Slider::new(&mut self.length, 0.1..=100.0).text("length"));
        let width = ui.add(Slider::new(&mut self.width, 0.1..=100.0).text("width"));
        let width_factor =
            ui.add(Slider::new(&mut self.width_factor, -5.0..=5.0).text("width factor"));

        if [n_segments, length, width, width_factor]
            .iter()
            .any(eframe::egui::Response::changed)
        {
            self.regenerate = true;
        }
        egui::reset_button(ui, self);
    }

    /// Updates segments properties.
    fn regenerate(&mut self, target_pos: Pos2) {
        self.segments
            .iter_mut()
            .enumerate()
            .fold(target_pos, |prev_pos, (n, segment)| {
                segment.update_dimensions(self.length, self.width + (self.width_factor * n as f32));
                segment.follow(prev_pos);
                segment.start()
            });

        let prev_segments_amount = self.segments.len();
        match prev_segments_amount.cmp(&self.segments_amount) {
            Ordering::Less => {
                self.segments
                    .extend((prev_segments_amount..self.segments_amount).map(|n| {
                        Segment::new(
                            pos2(self.length * n as f32, 0.0),
                            self.length,
                            self.width + (self.width_factor * n as f32),
                        )
                    }));
            }
            Ordering::Greater => {
                self.segments.truncate(self.segments_amount);
            }
            Ordering::Equal => {}
        }

        self.regenerate = false;
    }
}

impl App for KinematicsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(Visuals::dark());

            let cursor_pos = ctx.input(|i| i.pointer.interact_pos());

            if let Some(pos) = cursor_pos {
                self.main_ui(ui, pos);
            } else {
                ui.ctx().request_repaint();

                // go towards left lower corner until it disappears
                let add = if self
                    .segments
                    .last()
                    .map_or(true, |segment| segment.end().x > 0.0)
                {
                    vec2(-1.0, 1.0)
                } else {
                    Vec2::ZERO
                };
                self.main_ui(ui, self.current_target_pos + add);
            }
        });
    }
}
