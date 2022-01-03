use eframe::egui::{pos2, Color32, Pos2, Shape};

#[derive(PartialEq, Debug)]
pub struct Segment {
    start: Pos2,
    end: Pos2,
    length: f32,
    width: f32,
    angle: f32,
}

impl Segment {
    pub fn new(start: Pos2, length: f32, width: f32) -> Self {
        let mut segment = Segment {
            start,
            end: Pos2::ZERO,
            length,
            angle: 0.0,
            width,
        };
        segment.calculate_end();
        segment
    }

    pub fn follow(&mut self, target: Pos2) {
        let mut direction = target - self.start;
        self.angle = direction.y.atan2(direction.x);

        let magnitude = (direction.x.powi(2) + direction.y.powi(2)).sqrt();
        if magnitude != 0.0 && magnitude != 1.0 {
            direction.x /= magnitude;
            direction.y /= magnitude;
        }
        direction *= -1.0 * self.length;
        self.start = target + direction;
    }

    pub fn update(&mut self) {
        self.calculate_end();
    }

    pub fn show(&self) -> Shape {
        let line = [self.start, self.end];

        Shape::line_segment(line, (self.width, Color32::WHITE))
    }

    pub fn calculate_end(&mut self) {
        self.end = pos2(
            self.start.x + self.length * self.angle.cos(),
            self.start.y + self.length * self.angle.sin(),
        );
    }

    pub fn start(&self) -> Pos2 {
        self.start
    }

    pub fn end(&self) -> Pos2 {
        self.end
    }
}
