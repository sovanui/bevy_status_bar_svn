use bevy::prelude::*;
use crate::percentage::{AsPercentage, Percentage};

#[derive(Component, Copy, Clone)]
pub struct Health {
    max: u32,
    current: u32,
}


impl Health {

    pub fn new(max: u32) -> Self {
        Health {
            max,
            current: max,
        }
    }

    pub fn add(&mut self, value: u32) {
        self.current = u32::min(self.current + value, self.max);
    }

    pub fn remove(&mut self, value: u32) {
        self.current = if value > self.current { 0 } else { self.current - value };
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }
}


impl AsPercentage for Health {
    fn percentage(&self) -> Percentage {
        Percentage::new(self.current as f32 / self.max as f32)
    }
}
