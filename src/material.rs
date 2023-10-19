use bevy::prelude::{Color, Material};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "b48cb803-c227-4d47-aeb6-e56e0f2d4412"]
pub struct StatusBarMaterial {
    #[uniform(0)]
    pub foreground_color: Color,
    #[uniform(0)]
    pub background_color: Color,
    #[uniform(0)]
    pub percent: f32,
}

impl Material for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/percent-bar/material.wgsl".into()
    }
}