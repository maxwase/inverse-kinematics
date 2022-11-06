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
        segment.update();
        segment
    }

    /// Updates width and length of the segment.
    pub fn update_dimensions(&mut self, length: f32, width: f32) {
        self.length = length;
        self.width = width;
    }

    /// Updates segment's direction in order to follow the target.
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

        self.update();
    }

    /// Calculates new segment's end coordinates.
    fn update(&mut self) {
        self.end = pos2(
            self.start.x + self.length * self.angle.cos(),
            self.start.y + self.length * self.angle.sin(),
        );
    }

    /// Creates a [Shape] from the segment.
    pub fn show(&self) -> Shape {
        let line = [self.start, self.end];

        Shape::line_segment(line, (self.width, Color32::WHITE))
    }

    /// Segment's start.
    pub fn start(&self) -> Pos2 {
        self.start
    }

    /// Segment's end.
    pub fn end(&self) -> Pos2 {
        self.end
    }
}
