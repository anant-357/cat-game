use crate::state::State;
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Cat {
    pub x: f32,
    pub y: f32,
    pub mode: CatMode,
}

#[derive(Default)]
enum CatMode {
    #[default]
    Normal,
    Black,
    White,
}

pub fn setup_cat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Cat"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(122, 100, 21))),
        Transform::from_xyz(-1.0, 0.5, 0.0),
        Cat {
            x: -1.0,
            y: 0.5,
            ..Default::default()
        },
        StateScoped(State::Playing),
    ));
}
