use iced::{Point, Vector};

#[derive(Copy, Clone)]
pub struct Zoom {
    pub scale: f32,
    pub shift: Vector
}

impl Default for Zoom {
    fn default () -> Self {
        Self {
            scale: 1.0,
            shift: Vector::new(0.0, 0.0)
        }
    }
}

impl Zoom {
    /// Creates a new [Point]. Adds shift and multiplies coordinates by scale.
    pub fn apply (&self, point: Point) -> Point {
        let mut result = point;
        result = result - self.shift;
        result.x *= self.scale;
        result.y *= self.scale;
        result
    }
    /// Draws back the effect of [Zoom::apply].
    pub fn reverse(&self, point: Point) -> Point {
        let mut result = point;
        result.x /= self.scale;
        result.y /= self.scale;
        result = result + self.shift;
        result
    }
}
