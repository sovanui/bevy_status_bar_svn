use bevy::asset::Asset;
use bevy::prelude::{Color, Handle, Material, Shader};
use bevy::reflect::{TypePath};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};


pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11079857277321826659);


#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
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
        BAR_SHADER_HANDLE.into()
    }
}