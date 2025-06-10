use bevy::asset::{weak_handle, Asset};
use bevy::color::LinearRgba;
use bevy::prelude::{Handle, Material, Shader};
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("5790c741-54e1-4e5b-ac95-3475e3905a95");

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StatusBarMaterial {
    #[uniform(0)]
    pub foreground_color: LinearRgba,
    #[uniform(0)]
    pub background_color: LinearRgba,
    #[uniform(0)]
    pub percent: f32,
}

impl Material for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef {
        BAR_SHADER_HANDLE.into()
    }
}
