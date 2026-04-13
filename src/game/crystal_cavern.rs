use bevy::{
    color::LinearRgba,
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
};

use crate::{
    game::{
        area::{Area, GameEntity, SelectedArea},
        interactables::{Collider, CrystalNode, Interactable, Lightable},
    },
    state::State,
};

#[derive(Component)]
struct CrystalObject;

fn setup_crystal_cavern(
    existing: Query<(), With<CrystalObject>>,
    selected: Res<SelectedArea>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if selected.0 != Area::CrystalCavern { return; }
    if !existing.is_empty() { return; }

    // ── Material palette ────────────────────────────────────────────────────
    let spire_mats = [
        // Deep blue-purple
        materials.add(StandardMaterial {
            base_color: Color::srgba(0.28, 0.08, 0.75, 0.88),
            emissive: LinearRgba::rgb(0.20, 0.0, 0.9),
            perceptual_roughness: 0.08,
            reflectance: 0.95,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        // Teal-violet
        materials.add(StandardMaterial {
            base_color: Color::srgba(0.10, 0.25, 0.85, 0.80),
            emissive: LinearRgba::rgb(0.05, 0.10, 0.80),
            perceptual_roughness: 0.05,
            reflectance: 0.98,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        // Pale amethyst
        materials.add(StandardMaterial {
            base_color: Color::srgba(0.55, 0.20, 0.90, 0.75),
            emissive: LinearRgba::rgb(0.30, 0.05, 0.50),
            perceptual_roughness: 0.10,
            reflectance: 0.92,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    ];

    let ceiling_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.04, 0.03, 0.08),
        perceptual_roughness: 0.95,
        ..default()
    });

    let platform_mat = spire_mats[0].clone();

    // ── Outer ring: 16 spires at radius 12.5, heights 6–11 ─────────────────
    let outer_spires: &[(f32, f32, f32)] = &[
        // (angle_deg, width, height) — radius fixed at 12.5
        (  0.0, 0.45, 10.5),
        ( 22.5, 0.30,  7.0),
        ( 45.0, 0.55, 11.0),
        ( 67.5, 0.35,  7.8),
        ( 90.0, 0.40,  9.5),
        (112.5, 0.28,  6.5),
        (135.0, 0.50, 10.8),
        (157.5, 0.32,  7.5),
        (180.0, 0.45, 10.0),
        (202.5, 0.30,  7.2),
        (225.0, 0.58, 11.5),
        (247.5, 0.38,  8.0),
        (270.0, 0.42,  9.8),
        (292.5, 0.28,  6.8),
        (315.0, 0.48, 10.2),
        (337.5, 0.33,  7.6),
    ];
    for (i, &(angle_deg, width, height)) in outer_spires.iter().enumerate() {
        let angle = angle_deg.to_radians();
        let x = 12.5 * angle.cos();
        let z = 12.5 * angle.sin();
        let mat = spire_mats[i % 3].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("OuterSpire"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: width * 1.2,
                radius_top: width * 0.05,
                height,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(x, height / 2.0, z),
        ));
        commands.spawn((
            GameEntity, CrystalObject, Name::new("OuterSpireCap"),
            Mesh3d(meshes.add(Sphere::new(width * 0.14))),
            MeshMaterial3d(mat),
            Transform::from_xyz(x, height + width * 0.1, z),
        ));
    }

    // ── Mid ring: 12 spires at radius 8.0, heights 5–9 ─────────────────────
    let mid_spires: &[(f32, f32, f32)] = &[
        (  0.0, 0.35, 8.5),
        ( 30.0, 0.25, 6.0),
        ( 60.0, 0.40, 9.0),
        ( 90.0, 0.28, 6.5),
        (120.0, 0.38, 8.0),
        (150.0, 0.22, 5.5),
        (180.0, 0.42, 8.8),
        (210.0, 0.30, 6.2),
        (240.0, 0.36, 7.5),
        (270.0, 0.24, 5.8),
        (300.0, 0.45, 9.2),
        (330.0, 0.28, 6.8),
    ];
    for (i, &(angle_deg, width, height)) in mid_spires.iter().enumerate() {
        let angle = angle_deg.to_radians();
        let x = 8.0 * angle.cos();
        let z = 8.0 * angle.sin();
        let mat = spire_mats[i % 3].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("MidSpire"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: width * 1.2,
                radius_top: width * 0.06,
                height,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(x, height / 2.0, z),
        ));
        commands.spawn((
            GameEntity, CrystalObject, Name::new("MidSpireCap"),
            Mesh3d(meshes.add(Sphere::new(width * 0.14))),
            MeshMaterial3d(mat),
            Transform::from_xyz(x, height + width * 0.1, z),
        ));
    }

    // ── Inner ring: 8 spires at radius 4.5, heights 4–7 ────────────────────
    let inner_spires: &[(f32, f32, f32)] = &[
        (  0.0, 0.25, 6.5),
        ( 45.0, 0.18, 4.5),
        ( 90.0, 0.28, 7.0),
        (135.0, 0.20, 5.0),
        (180.0, 0.24, 6.2),
        (225.0, 0.16, 4.2),
        (270.0, 0.30, 6.8),
        (315.0, 0.22, 5.5),
    ];
    for (i, &(angle_deg, width, height)) in inner_spires.iter().enumerate() {
        let angle = angle_deg.to_radians();
        let x = 4.5 * angle.cos();
        let z = 4.5 * angle.sin();
        let mat = spire_mats[i % 3].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("InnerSpire"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: width * 1.2,
                radius_top: width * 0.07,
                height,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(x, height / 2.0, z),
        ));
        commands.spawn((
            GameEntity, CrystalObject, Name::new("InnerSpireCap"),
            Mesh3d(meshes.add(Sphere::new(width * 0.15))),
            MeshMaterial3d(mat),
            Transform::from_xyz(x, height + width * 0.1, z),
        ));
    }

    // ── Central crystal cluster ─────────────────────────────────────────────
    let cluster: &[(f32, f32, f32, f32)] = &[
        // (x, z, width, height)
        ( 0.0,  0.0, 0.55, 16.0),  // tallest center spike — reaches toward ceiling
        ( 0.6,  0.3, 0.35,  9.5),
        (-0.5,  0.5, 0.28,  8.8),
        ( 0.3, -0.7, 0.40, 10.5),
        (-0.7, -0.3, 0.22,  7.5),
        ( 0.8, -0.2, 0.30,  9.0),
        (-0.3,  0.8, 0.25,  8.2),
    ];
    for (i, &(x, z, width, height)) in cluster.iter().enumerate() {
        let mat = spire_mats[i % 3].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("ClusterSpire"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: width * 1.1,
                radius_top: width * 0.04,
                height,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(x, height / 2.0, z),
        ));
        commands.spawn((
            GameEntity, CrystalObject, Name::new("ClusterCap"),
            Mesh3d(meshes.add(Sphere::new(width * 0.18))),
            MeshMaterial3d(mat),
            Transform::from_xyz(x, height + width * 0.15, z),
        ));
    }

    // ── Crystal archways — 4 cardinal directions at r≈6 ────────────────────
    let archway_angles: &[f32] = &[0.0, 90.0, 180.0, 270.0];
    for &angle_deg in archway_angles {
        let angle = angle_deg.to_radians();
        let cx = 6.5 * angle.cos();
        let cz = 6.5 * angle.sin();
        // Tangent direction for pillar offset
        let tx = -angle.sin() * 0.9;
        let tz =  angle.cos() * 0.9;
        let mat = spire_mats[0].clone();
        let pillar_h = 9.0_f32;
        let pillar_w = 0.22_f32;
        // Left pillar
        commands.spawn((
            GameEntity, CrystalObject, Name::new("ArchPillarL"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: pillar_w * 1.2,
                radius_top: pillar_w * 0.1,
                height: pillar_h,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(cx + tx, pillar_h / 2.0, cz + tz),
        ));
        // Right pillar
        commands.spawn((
            GameEntity, CrystalObject, Name::new("ArchPillarR"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: pillar_w * 1.2,
                radius_top: pillar_w * 0.1,
                height: pillar_h,
            })),
            MeshMaterial3d(mat.clone()),
            Transform::from_xyz(cx - tx, pillar_h / 2.0, cz - tz),
        ));
        // Lintel — thin cuboid connecting the two pillars at the top
        let lintel_mat = spire_mats[1].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("ArchLintel"),
            Mesh3d(meshes.add(Cuboid::new(1.8, 0.2, 0.2))),
            MeshMaterial3d(lintel_mat),
            Transform {
                translation: Vec3::new(cx, pillar_h + 0.1, cz),
                rotation: Quat::from_rotation_y(angle),
                ..default()
            },
        ));
    }

    // ── Elevated platforms — 3 crystal disc platforms at mid-radius ─────────
    let platforms: &[(f32, f32, f32)] = &[
        ( 5.0,  5.0, 1.5),
        (-6.0,  2.0, 1.2),
        ( 2.0, -5.5, 1.8),
    ];
    for &(x, z, radius) in platforms {
        commands.spawn((
            GameEntity, CrystalObject, Name::new("CrystalPlatform"),
            Mesh3d(meshes.add(Circle::new(radius))),
            MeshMaterial3d(platform_mat.clone()),
            Transform {
                translation: Vec3::new(x, 3.0, z),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..default()
            },
        ));
        // Small decorative shard cluster on each platform
        for k in 0..3_u32 {
            let shard_angle = (k as f32) * std::f32::consts::TAU / 3.0;
            let sx = x + (radius * 0.5) * shard_angle.cos();
            let sz = z + (radius * 0.5) * shard_angle.sin();
            let sh = 0.5 + (k as f32) * 0.15;
            commands.spawn((
                GameEntity, CrystalObject, Name::new("PlatformShard"),
                Mesh3d(meshes.add(ConicalFrustum {
                    radius_bottom: 0.06,
                    radius_top: 0.01,
                    height: sh,
                })),
                MeshMaterial3d(spire_mats[k as usize % 3].clone()),
                Transform::from_xyz(sx, 3.0 + sh / 2.0, sz),
            ));
        }
    }

    // ── Floor shards — 24 total spread over 12-unit radius ─────────────────
    let floor_shards: &[(f32, f32, f32, f32)] = &[
        // Near center (original 8)
        ( 1.5, -1.0, 0.12, 0.50),
        (-0.8,  1.8, 0.09, 0.40),
        ( 2.5,  0.5, 0.14, 0.60),
        (-2.0, -0.5, 0.10, 0.30),
        ( 0.3,  2.5, 0.08, 0.40),
        (-1.5, -2.0, 0.13, 0.50),
        ( 2.0, -2.2, 0.11, 0.45),
        (-0.5, -1.5, 0.09, 0.35),
        // Mid distance (new 16)
        ( 5.5, -3.0, 0.18, 0.90),
        (-4.0,  6.0, 0.14, 0.70),
        ( 7.0,  2.5, 0.20, 1.10),
        (-6.5, -4.0, 0.16, 0.80),
        ( 3.0,  7.5, 0.22, 1.30),
        (-7.5,  1.5, 0.12, 0.60),
        ( 8.0, -1.0, 0.18, 0.90),
        ( 1.5, -8.0, 0.15, 0.70),
        (-3.5, -7.0, 0.20, 1.00),
        ( 6.5,  5.5, 0.17, 0.80),
        (-5.0,  6.5, 0.14, 0.60),
        ( 9.0,  3.0, 0.22, 1.20),
        (-9.5, -2.0, 0.18, 0.90),
        ( 4.5, -8.5, 0.16, 0.70),
        (-8.0,  5.0, 0.20, 1.00),
        ( 7.5, -6.0, 0.14, 0.60),
    ];
    for (i, &(x, z, width, height)) in floor_shards.iter().enumerate() {
        let mat = spire_mats[i % 3].clone();
        commands.spawn((
            GameEntity, CrystalObject, Name::new("FloorShard"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: width * 1.1,
                radius_top: width * 0.05,
                height,
            })),
            MeshMaterial3d(mat),
            Transform::from_xyz(x, height / 2.0, z),
        ));
    }

    // ── Dark ceiling disc at y=18 ───────────────────────────────────────────
    commands.spawn((
        GameEntity, CrystalObject, Name::new("CrystalCeiling"),
        Mesh3d(meshes.add(Circle::new(16.0))),
        MeshMaterial3d(ceiling_mat),
        Transform {
            translation: Vec3::new(0.0, 18.0, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ..default()
        },
    ));

    // ── 10 point lights spread across the cavern ───────────────────────────
    let glow_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.3, 1.0),
        emissive: LinearRgba::rgb(1.5, 0.5, 3.0),
        ..default()
    });

    // Inner 4 at radius 3, y=1.5 — 2 with shadows
    let inner_lights: &[(f32, f32)] = &[
        ( 3.0,  0.0),
        ( 0.0,  3.0),
        (-3.0,  0.0),
        ( 0.0, -3.0),
    ];
    for (i, &(x, z)) in inner_lights.iter().enumerate() {
        let point_light = PointLight {
            color: Color::srgb(0.55, 0.25, 1.0),
            intensity: 120_000.0,
            range: 10.0,
            shadows_enabled: i < 2,
            ..default()
        };
        commands.spawn((
            GameEntity, CrystalObject, Name::new("CrystalGlow"),
            Mesh3d(meshes.add(Sphere::new(0.08))),
            MeshMaterial3d(glow_mat.clone()),
            Transform::from_xyz(x, 1.5, z),
        )).with_child((point_light, Transform::default()));
    }

    // Mid 4 at radius 7, y=2.0 — no shadows
    let mid_lights: &[(f32, f32)] = &[
        ( 7.0,  0.0),
        ( 0.0,  7.0),
        (-7.0,  0.0),
        ( 0.0, -7.0),
    ];
    for &(x, z) in mid_lights {
        commands.spawn((
            GameEntity, CrystalObject, Name::new("MidGlow"),
            Mesh3d(meshes.add(Sphere::new(0.08))),
            MeshMaterial3d(glow_mat.clone()),
            Transform::from_xyz(x, 2.0, z),
        )).with_child((
            PointLight {
                color: Color::srgb(0.55, 0.25, 1.0),
                intensity: 90_000.0,
                range: 12.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::default(),
        ));
    }

    // Outer 2 at radius 11, y=3.0 — no shadows
    let cos45 = std::f32::consts::FRAC_PI_4.cos();
    let outer_lights: &[(f32, f32)] = &[
        ( 11.0 * cos45,  11.0 * cos45),
        (-11.0 * cos45, -11.0 * cos45),
    ];
    for &(x, z) in outer_lights {
        commands.spawn((
            GameEntity, CrystalObject, Name::new("OuterGlow"),
            Mesh3d(meshes.add(Sphere::new(0.08))),
            MeshMaterial3d(glow_mat.clone()),
            Transform::from_xyz(x, 3.0, z),
        )).with_child((
            PointLight {
                color: Color::srgb(0.55, 0.25, 1.0),
                intensity: 70_000.0,
                range: 14.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::default(),
        ));
    }

    // Cool blue-purple fill light
    commands.spawn((
        GameEntity, CrystalObject,
        DirectionalLight {
            color: Color::srgb(0.15, 0.08, 0.35),
            illuminance: 80.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, -0.5, 0.0)),
    ));

    // ── 4 crystal node objectives at cardinal positions, radius 6 ───────────
    let node_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.40, 0.10, 0.90, 0.90),
        emissive: LinearRgba::rgb(0.30, 0.05, 1.20),
        perceptual_roughness: 0.05,
        reflectance: 0.98,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let node_positions: &[(f32, f32)] = &[
        ( 6.0,  0.0),
        ( 0.0,  6.0),
        (-6.0,  0.0),
        ( 0.0, -6.0),
    ];
    for &(x, z) in node_positions {
        let node_height = 2.5_f32;
        commands.spawn((
            GameEntity, CrystalObject, CrystalNode,
            Name::new("CrystalNodeBody"),
            Mesh3d(meshes.add(ConicalFrustum {
                radius_bottom: 0.30,
                radius_top: 0.05,
                height: node_height,
            })),
            MeshMaterial3d(node_mat.clone()),
            Transform::from_xyz(x, node_height / 2.0, z),
            Interactable { radius: 2.0 },
            Collider { radius: 0.32 },
            Lightable { lit: false },
        ));
    }
}

fn apply_crystal_fog(
    selected: Res<SelectedArea>,
    mut cameras: Query<&mut DistanceFog>,
) {
    if selected.0 != Area::CrystalCavern { return; }
    for mut fog in &mut cameras {
        fog.color = Color::srgb(0.02, 0.01, 0.06);
        fog.falloff = FogFalloff::Exponential { density: 0.05 };
    }
}

pub struct CrystalCavernPlugin;

impl Plugin for CrystalCavernPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(State::Playing),
            (setup_crystal_cavern, apply_crystal_fog),
        );
    }
}
