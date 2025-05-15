use bevy::prelude::{
    Assets, Circle, Color, Commands, Mesh, Mesh3d, MeshMaterial3d, Quat, ResMut, StandardMaterial,
    States, Transform,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum Area {
    #[default]
    Cave,
}

pub fn setup_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 235, 205))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}
