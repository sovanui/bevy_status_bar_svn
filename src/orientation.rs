use bevy::prelude::Quat;

pub enum Orientation {
    FacingCamera,
    Static(Quat),
    Inherit
}