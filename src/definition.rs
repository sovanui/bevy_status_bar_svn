use std::marker::PhantomData;

use crate::plugin::PercentageComponent;
use bevy::{color::palettes::css::{LIME, RED}, prelude::*};

#[derive(Component)]
pub struct StatusBarDefinition<T: PercentageComponent> {
    pub size: Size,
    pub offset: Vec3,
    pub orientation: Orientation,
    pub direction: Direction,
    pub foreground_color: Color,
    pub background_color: Color,
    pub phantom_data: PhantomData<T>,
}

impl<T: PercentageComponent> Default for StatusBarDefinition<T> {
    fn default() -> Self {
        Self {
            size: Size::new(1.2, 0.20),
            offset: Vec3::new(0.0, 0.9, -0.7),
            orientation: Orientation::FacingCamera,
            direction: Direction::Horizontal,
            foreground_color: Color::Srgba(LIME),
            background_color: Color::Srgba(RED),
            phantom_data: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size {
            width: f32::max(width, 0.0),
            height: f32::max(height, 0.0),
        }
    }

    pub fn width(&self) -> f32 { self.width }
    pub fn height(&self) -> f32 { self.height }
}

#[derive(Default)]
pub enum Orientation {
    #[default]
    FacingCamera,
    // Static(Quat),
    // Inherit
}

#[derive(Default)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical
}
