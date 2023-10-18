use bevy::prelude::{Color, Component, Vec3};
use crate::direction::Direction;
use crate::orientation::Orientation;
use crate::size::Size;

#[derive(Component)]
pub struct StatusBarDefinition {
    size: Size,
    offset: Vec3,
    orientation: Orientation,
    direction: Direction,
    foreground_color: Color,
    background_color: Color,
}
