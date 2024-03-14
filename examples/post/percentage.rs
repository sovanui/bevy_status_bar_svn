#[derive(Clone, Copy)]
pub struct Percentage {
    value: f32
}


impl Percentage {
    pub fn new(value: f32) -> Self {
        Self { value: Self::clamp(value) }
    }

    pub fn set(&mut self, value: f32) { self.value = Self::clamp(value); }
    pub fn value(&self) -> f32 { self.value }

    fn clamp(value: f32) -> f32 {
        if value < 0.0 { 0.0 }
        else if value > 100.0 { 100.0 }
        else { value }
    }
}


pub trait AsPercentage {
    fn percentage(&self) -> Percentage;
}


impl AsPercentage for Percentage {
    fn percentage(&self) -> Percentage { *self }
}