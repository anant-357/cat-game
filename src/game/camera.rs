use crate::{characters::Cat, settings::AppSettings};
use crate::game::area::{AreaBounds, GameEntity};
use bevy::post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::prelude::{
    ButtonInput, Camera3d, Color, Commands, Component, KeyCode, Msaa, ParamSet, PointLight, Query,
    Res, Time, Transform, Vec2, Vec3, With, default,
};

use bevy::input::mouse::AccumulatedMouseMotion;

#[derive(Component, Clone, Copy)]
pub struct CameraRig {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

#[derive(Component)]
pub struct CameraMarkerComponent;

pub fn setup_camera(
    existing: Query<(), With<CameraMarkerComponent>>,
    mut commands: Commands,
) {
    if !existing.is_empty() {
        return;
    }
    // Headlamp — fills the scene with enough ambient to see; embers handle dramatic shadows
    commands.spawn((
        GameEntity,
        CameraMarkerComponent,
        PointLight {
            intensity: 80_000.0,
            range: 22.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        GameEntity,
        CameraMarkerComponent,
        Camera3d::default(),
        CameraRig {
            yaw: 0.0,
            pitch: 0.3,
            distance: 10.0,
        },
        Msaa::Sample4,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.28,
            low_frequency_boost: 0.5,
            low_frequency_boost_curvature: 0.5,
            high_pass_frequency: 1.0,
            prefilter: BloomPrefilter {
                threshold: 0.4,
                threshold_softness: 0.3,
            },
            composite_mode: BloomCompositeMode::Additive,
            ..Bloom::NATURAL
        },
        DistanceFog {
            color: Color::srgb(0.04, 0.03, 0.02),
            falloff: FogFalloff::Exponential { density: 0.12 },
            ..default()
        },
    ));
}

pub fn orbit_camera_keyboard(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    bounds: Res<AreaBounds>,
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
    let pitch_limit = 0.7;

    if keys.pressed(KeyCode::ArrowLeft) && !keys.pressed(KeyCode::ArrowRight) {
        rig.yaw += rotate_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) && !keys.pressed(KeyCode::ArrowLeft) {
        rig.yaw -= rotate_speed * time.delta_secs();
    }

    if keys.pressed(KeyCode::ArrowUp) && !keys.pressed(KeyCode::ArrowDown) {
        rig.pitch = (rig.pitch + rotate_speed * time.delta_secs()).clamp(0.15, pitch_limit);
    }

    if keys.pressed(KeyCode::ArrowDown) && !keys.pressed(KeyCode::ArrowUp) {
        rig.pitch = (rig.pitch - rotate_speed * time.delta_secs()).clamp(0.15, pitch_limit);
    }

    let offset = Vec3::new(
        rig.distance * rig.pitch.cos() * rig.yaw.cos(),
        rig.distance * rig.pitch.sin(),
        rig.distance * rig.pitch.cos() * rig.yaw.sin(),
    );

    camera_transform.translation = target_transform_translation + offset;

    // Keep camera inside the area cylinder
    let cam_xz = Vec2::new(camera_transform.translation.x, camera_transform.translation.z);
    if cam_xz.length() > bounds.camera_radius {
        let clamped = cam_xz.normalize() * bounds.camera_radius;
        camera_transform.translation.x = clamped.x;
        camera_transform.translation.z = clamped.y;
    }

    camera_transform.look_at(target_transform_translation, Vec3::Y);
}

pub fn orbit_camera_mouse(
    mut set: ParamSet<(
        Query<(&mut CameraRig, &mut Transform)>,
        Query<&Transform, With<Cat>>,
    )>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    settings: Res<AppSettings>,
    bounds: Res<AreaBounds>,
) {
    let target_transform_translation = set
        .p1()
        .single()
        .expect("Target not cat?")
        .translation
        .clone();
    let mut query = set.p0();
    let (mut rig, mut camera_transform) = query.single_mut().expect("Camera not spawned");

    let rotate_speed = 0.01;
    let pitch_limit = 0.7;
    let invert = if settings.invert_mouse { -1.0 } else { 1.0 };

    rig.yaw = rig.yaw + rotate_speed * accumulated_mouse_motion.delta.x;
    rig.pitch =
        (rig.pitch + invert * rotate_speed * accumulated_mouse_motion.delta.y).clamp(0.15, pitch_limit);

    let offset = Vec3::new(
        rig.distance * rig.pitch.cos() * rig.yaw.cos(),
        rig.distance * rig.pitch.sin(),
        rig.distance * rig.pitch.cos() * rig.yaw.sin(),
    );

    camera_transform.translation = target_transform_translation + offset;

    // Keep camera inside the area cylinder
    let cam_xz = Vec2::new(camera_transform.translation.x, camera_transform.translation.z);
    if cam_xz.length() > bounds.camera_radius {
        let clamped = cam_xz.normalize() * bounds.camera_radius;
        camera_transform.translation.x = clamped.x;
        camera_transform.translation.z = clamped.y;
    }

    camera_transform.look_at(target_transform_translation, Vec3::Y);
}
