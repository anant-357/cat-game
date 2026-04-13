use bevy::{
    camera::Camera3d,
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    },
    ecs::{component::Component, query::QueryItem},
    prelude::{App, Commands, Entity, OnEnter, OnExit, Plugin, Query, With},
    render::{
        extract_component::ExtractComponent,
        render_graph::{InternedRenderLabel, InternedRenderSubGraph, RenderLabel, RenderSubGraph},
        render_resource::ShaderType,
    },
    shader::ShaderRef,
};

use crate::state::State;

#[derive(Component, Clone, Copy, Default, ShaderType)]
pub struct PauseBlur {
    /// Controls the blur radius in texels. 0 = no blur, 8+ = heavy blur.
    pub intensity: f32,
}

impl ExtractComponent for PauseBlur {
    type QueryData = &'static PauseBlur;
    type QueryFilter = ();
    type Out = PauseBlur;

    fn extract_component(item: QueryItem<Self::QueryData>) -> Option<Self::Out> {
        Some(*item)
    }
}

impl FullscreenMaterial for PauseBlur {
    fn fragment_shader() -> ShaderRef {
        "shaders/pause_blur.wgsl".into()
    }

    // Register the node once in the Core3d graph at startup so it is always
    // present. The ViewNode only processes cameras that have DynamicUniformIndex<PauseBlur>,
    // which exists only while the PauseBlur component is on the camera entity.
    fn sub_graph() -> Option<InternedRenderSubGraph> {
        Some(Core3d.intern())
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}

fn add_blur(mut commands: Commands, cameras: Query<Entity, With<Camera3d>>) {
    for entity in &cameras {
        commands.entity(entity).insert(PauseBlur { intensity: 8.0 });
    }
}

fn remove_blur(mut commands: Commands, cameras: Query<Entity, With<PauseBlur>>) {
    for entity in &cameras {
        commands.entity(entity).remove::<PauseBlur>();
    }
}

pub struct BlurPlugin;
impl Plugin for BlurPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<PauseBlur>::default())
            .add_systems(OnEnter(State::Paused), add_blur)
            .add_systems(OnExit(State::Paused), remove_blur);
    }
}
