use bevy::prelude::*;

#[derive(Component)]
pub struct StatusBarDefinition {
    pub size: Size,
    pub offset: Vec3,
    pub orientation: Orientation,
    pub direction: Direction,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Default for StatusBarDefinition {
    fn default() -> Self {
        Self {
            size: Size::new(1.2, 0.20),
            offset: Vec3::new(0.0, 0.9, -0.7),
            orientation: Orientation::FacingCamera,
            direction: Direction::Horizontal,
            foreground_color: Color::GREEN,
            background_color: Color::RED,
        }
    }
}


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

pub enum Orientation {
    FacingCamera,
    // Static(Quat),
    // Inherit
}

pub enum Direction {
    Horizontal,
    // Vertical
}