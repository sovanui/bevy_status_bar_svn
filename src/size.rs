use std::cmp::max;

pub struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size {
            width: f32::max(width, 0.0),
            height: f32::max(height, 0.0)
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}