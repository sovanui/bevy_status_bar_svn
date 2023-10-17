
#[derive(Clone, Copy)]
pub struct Percentage {
    value: f32
}


impl Percentage {
    pub fn new(value: f32) -> Self {
        if value < 0.0 { Self { value: 0.0 } }
        else if value > 100.0 { Self { value: 100.0 } }
        else { Self { value } }
    }

    pub fn values(&self) -> f32 { self.value }
}


pub trait AsPercentage {
    fn percentage(&self) -> Percentage;
}


impl AsPercentage for Percentage {
    fn percentage(&self) -> Percentage { *self }
}
