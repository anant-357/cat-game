use bevy::prelude::{
    Assets, Circle, Color, Commands, Component, Mesh, Mesh3d, MeshMaterial3d, Query, Quat, Res,
    ResMut, Resource, StandardMaterial, States, Transform, With,
};
use strum::{EnumCount, EnumIter, EnumString, IntoStaticStr};

use crate::render::{RockExtension, RockMaterial};

/// Marker for all entities that belong to the game world.
/// Despawned when returning to the main menu so pausing preserves them.
#[derive(Component)]
pub struct GameEntity;

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, EnumIter, IntoStaticStr, EnumCount, EnumString,
)]
pub enum Area {
    #[default]
    Cave,
    CrystalCavern,
}

/// Tracks which area the player selected from the area menu.
#[derive(Resource, Default)]
pub struct SelectedArea(pub Area);

/// Per-area boundary radii — set by `apply_area_bounds` on `OnEnter(State::Playing)`.
#[derive(Resource)]
pub struct AreaBounds {
    /// XZ radius for cat movement and object clamping.
    pub play_radius: f32,
    /// XZ radius for the camera orbit clamp.
    pub camera_radius: f32,
}

impl Default for AreaBounds {
    fn default() -> Self {
        Self { play_radius: 3.5, camera_radius: 7.8 }
    }
}

/// Writes the correct `AreaBounds` values for the selected area.
/// Must run before any system that reads `AreaBounds`.
pub fn apply_area_bounds(
    selected: Res<SelectedArea>,
    mut bounds: ResMut<AreaBounds>,
) {
    match selected.0 {
        Area::Cave          => { bounds.play_radius = 3.5;  bounds.camera_radius = 7.8;  }
        Area::CrystalCavern => { bounds.play_radius = 12.5; bounds.camera_radius = 15.5; }
    }
}

pub fn setup_area(
    existing: Query<(), With<GameEntity>>,
    selected: Res<SelectedArea>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rock_materials: ResMut<Assets<RockMaterial>>,
) {
    if !existing.is_empty() {
        return;
    }
    let (base_color, noise_scale, floor_radius) = match selected.0 {
        Area::Cave          => (Color::srgb(0.22, 0.18, 0.15), 2.0, 4.0_f32),
        Area::CrystalCavern => (Color::srgb(0.08, 0.06, 0.14), 1.5, 14.0_f32),
    };
    commands.spawn((
        GameEntity,
        Mesh3d(meshes.add(Circle::new(floor_radius))),
        MeshMaterial3d(rock_materials.add(RockMaterial {
            base: StandardMaterial {
                base_color,
                perceptual_roughness: 0.95,
                ..Default::default()
            },
            extension: RockExtension::new(noise_scale),
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}
