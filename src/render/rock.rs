use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, MaterialPlugin},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

/// Extension for all cave rock surfaces.
/// `noise_scale` is packed in a Vec4 so the uniform buffer is always 16 bytes.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct RockExtension {
    /// x = noise_scale, yzw = padding
    #[uniform(100)]
    pub params: Vec4,
}

impl RockExtension {
    pub fn new(noise_scale: f32) -> Self {
        Self { params: Vec4::new(noise_scale, 0.0, 0.0, 0.0) }
    }
}

impl MaterialExtension for RockExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/rock_material.wgsl".into()
    }
}

pub type RockMaterial = ExtendedMaterial<StandardMaterial, RockExtension>;

pub struct RockMaterialPlugin;
impl Plugin for RockMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<RockMaterial>::default());
    }
}
