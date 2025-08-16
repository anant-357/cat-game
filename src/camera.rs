use crate::Cat;
use bevy::prelude::{
    ButtonInput, Camera3d, Commands, Component, KeyCode, ParamSet, PointLight, Query, Res, Time,
    Transform, Vec3, With, default,
};
#[derive(Component, Clone, Copy)]
pub struct CameraRig {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        CameraRig {
            yaw: 0.0,
            pitch: 0.3,
            distance: 10.0,
        },
    ));
}

pub fn orbit_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut CameraRig, &mut Transform)>,
        Query<&Transform, With<Cat>>,
    )>,
) {
    let target_transform_translation = set
        .p1()
        .single()
        .expect("Target not cat?")
        .translation
        .clone();
    let mut query = set.p0();
    let (mut rig, mut camera_transform) = query.single_mut().expect("Camera not spawned");

    let rotate_speed = 1.5;
    let pitch_limit = 1.0;

    if keys.pressed(KeyCode::ArrowLeft) && !keys.pressed(KeyCode::ArrowRight) {
        rig.yaw += rotate_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) && !keys.pressed(KeyCode::ArrowLeft) {
        rig.yaw -= rotate_speed * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowUp) && !keys.pressed(KeyCode::ArrowDown) {
        rig.pitch = (rig.pitch + rotate_speed * time.delta_secs()).clamp(0., pitch_limit);
    }

    if keys.pressed(KeyCode::ArrowDown) && !keys.pressed(KeyCode::ArrowUp) {
        rig.pitch = (rig.pitch - rotate_speed * time.delta_secs()).clamp(0., pitch_limit);
    }

    let offset = Vec3::new(
        rig.distance * rig.pitch.cos() * rig.yaw.cos(),
        rig.distance * rig.pitch.sin(),
        rig.distance * rig.pitch.cos() * rig.yaw.sin(),
    );

    camera_transform.translation = target_transform_translation + offset;
    camera_transform.look_at(target_transform_translation, Vec3::Y);
}
