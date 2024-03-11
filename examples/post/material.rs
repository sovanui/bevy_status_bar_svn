use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};


#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StatusBarMaterial {
    #[uniform(0)] pub foreground_color: Color,
    #[uniform(0)] pub background_color: Color,
    #[uniform(0)] pub percent: f32
}

impl Material for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef { "bar.wgsl".into() }
}