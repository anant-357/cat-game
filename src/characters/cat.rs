use crate::{game::camera::CameraRig, state::State};
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Cat {
    mode: CatMode,
    material: Handle<StandardMaterial>,
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
    let material = materials.add(Color::srgb_u8(122, 100, 21));
    let cat = Cat {
        material: material.clone(),
        ..Default::default()
    };
    commands.spawn((
        DespawnOnExit(State::Playing),
        Name::new("Cat"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(material),
        Transform::from_xyz(-1.0, 0.5, 0.0),
        cat,
    ));
}

pub fn change_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Cat>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut cat in &mut query {
            cat.mode = match cat.mode {
                CatMode::Black => CatMode::White,
                CatMode::White => CatMode::Black,
                CatMode::Normal => CatMode::White,
            };

            if let Some(material) = materials.get_mut(&cat.material) {
                material.base_color = match cat.mode {
                    CatMode::Black => Color::BLACK,
                    CatMode::White => Color::WHITE,
                    CatMode::Normal => Color::srgb_u8(122, 100, 21),
                }
            }
        }
    }
}

pub fn move_cat(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Query<&mut Transform, With<Cat>>,
        Query<&Transform, With<CameraRig>>,
    )>,
) {
    let rig = set.p1().single().expect("Camera Rig not spawned").clone();
    let mut query = set.p0();
    let pressed = keyboard_input.get_pressed();
    let mut input_direction = Vec3::ZERO;
    for key in pressed {
        match key {
            KeyCode::KeyW => input_direction.z += 1.,
            KeyCode::KeyS => input_direction.z -= 1.,
            KeyCode::KeyD => input_direction.x += 1.,
            KeyCode::KeyA => input_direction.x -= 1.,
            _ => (),
        }
    }
    if input_direction != Vec3::ZERO {
        let forward = rig.forward().with_y(0.).normalize();
        let right = rig.right().normalize();

        let move_direction = (input_direction.z * forward + input_direction.x * right).normalize();
        if let Some(mut transform) = query.iter_mut().next() {
            let speed = 0.1; // tweak movement speed here
            transform.translation += move_direction * speed;
        }
    }
}

pub fn exit_play(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(State::MainMenu);
    }
}
