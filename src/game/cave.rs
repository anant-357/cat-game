use bevy::{
    color::LinearRgba,
    prelude::*,
    render::render_resource::Face,
};

use crate::{
    game::{
        area::{Area, GameEntity, SelectedArea},
        interactables::{Collider, Interactable, Lightable},
    },
    render::{RockExtension, RockMaterial},
    state::State,
};

#[derive(Component)]
struct CaveObject;

fn rock_mat(
    base_color: Color,
    roughness: f32,
    noise_scale: f32,
    double_sided: bool,
    cull_mode: Option<Face>,
    materials: &mut Assets<RockMaterial>,
) -> Handle<RockMaterial> {
    materials.add(RockMaterial {
        base: StandardMaterial {
            base_color,
            perceptual_roughness: roughness,
            double_sided,
            cull_mode,
            ..default()
        },
        extension: RockExtension::new(noise_scale),
    })
}

fn setup_cave(
    existing: Query<(), With<CaveObject>>,
    selected: Res<SelectedArea>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut rock_materials: ResMut<Assets<RockMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    if selected.0 != Area::Cave { return; }
    if !existing.is_empty() {
        return;
    }

    let stone_mat = rock_mat(Color::srgb(0.25, 0.22, 0.18), 0.90, 3.0, false, None, &mut rock_materials);
    let ceiling_mat = rock_mat(Color::srgb(0.15, 0.12, 0.10), 0.88, 2.5, false, None, &mut rock_materials);
    let stalactite_mat = rock_mat(Color::srgb(0.30, 0.26, 0.22), 0.88, 3.5, false, None, &mut rock_materials);
    let floor_rock_mat = rock_mat(Color::srgb(0.35, 0.28, 0.22), 0.92, 4.0, false, None, &mut rock_materials);
    let wall_mat = rock_mat(Color::srgb(0.12, 0.10, 0.08), 0.85, 1.5, true, Some(Face::Front), &mut rock_materials);

    // 12 rock columns: (angle_deg, width, height, depth)
    let columns: &[(f32, f32, f32, f32)] = &[
        (  0.0, 1.2, 4.0, 1.0),
        ( 30.0, 0.9, 3.0, 0.8),
        ( 60.0, 1.4, 5.0, 1.1),
        ( 90.0, 1.0, 3.5, 0.9),
        (120.0, 1.3, 4.5, 1.2),
        (150.0, 0.8, 3.2, 0.7),
        (180.0, 1.5, 4.8, 1.0),
        (210.0, 1.0, 3.0, 1.1),
        (240.0, 1.2, 4.2, 0.9),
        (270.0, 0.9, 3.8, 0.8),
        (300.0, 1.4, 5.2, 1.2),
        (330.0, 1.1, 3.5, 1.0),
    ];

    for &(angle_deg, width, height, depth) in columns {
        let angle = angle_deg.to_radians();
        let x = 5.5 * angle.cos();
        let z = 5.5 * angle.sin();
        // Main column body — slightly rotated for organic feel
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("RockColumn"),
            Mesh3d(meshes.add(Cuboid::new(width, height, depth))),
            MeshMaterial3d(stone_mat.clone()),
            Transform::from_xyz(x, height / 2.0, z)
                .with_rotation(Quat::from_rotation_y(angle + 0.15)),
        ));
        // Narrow cap on top for a natural stacked-rock look
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("RockColumnCap"),
            Mesh3d(meshes.add(Cuboid::new(width * 0.65, 0.35, depth * 0.65))),
            MeshMaterial3d(stone_mat.clone()),
            Transform::from_xyz(x, height + 0.17, z),
        ));
    }

    // Ceiling disc at y = 7.5, facing downward
    commands.spawn((
        GameEntity,
        CaveObject,
        Name::new("CaveCeiling"),
        Mesh3d(meshes.add(Circle::new(7.0))),
        MeshMaterial3d(ceiling_mat),
        Transform {
            translation: Vec3::new(0.0, 7.5, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ..default()
        },
    ));

    // 10 stalactites: (x, z, radius, length)
    let stalactites: &[(f32, f32, f32, f32)] = &[
        ( 0.5,  1.0, 0.15, 1.8),
        (-1.5,  2.0, 0.12, 2.4),
        ( 2.5, -1.0, 0.10, 1.2),
        (-2.0, -2.5, 0.20, 2.0),
        ( 1.0,  2.5, 0.08, 1.5),
        (-3.0,  0.5, 0.18, 1.0),
        ( 3.5,  1.5, 0.12, 2.2),
        (-1.0, -1.5, 0.15, 1.6),
        ( 2.0, -2.0, 0.10, 1.0),
        (-2.5,  1.0, 0.22, 2.8),
    ];

    for &(x, z, radius, length) in stalactites {
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("Stalactite"),
            Mesh3d(meshes.add(Cone { radius, height: length })),
            MeshMaterial3d(stalactite_mat.clone()),
            Transform {
                translation: Vec3::new(x, 7.5 - length / 2.0, z),
                rotation: Quat::from_rotation_x(std::f32::consts::PI),
                ..default()
            },
        ));
    }

    // Stalagmites — rising cones from the floor
    let stalagmites: &[(f32, f32, f32, f32)] = &[
        ( 1.8, -0.5, 0.12, 0.8),
        (-0.5,  2.2, 0.08, 0.6),
        ( 3.0,  0.8, 0.10, 0.9),
        (-2.8, -1.5, 0.15, 1.1),
        ( 0.5, -2.8, 0.09, 0.7),
    ];

    for &(x, z, radius, height) in stalagmites {
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("Stalagmite"),
            Mesh3d(meshes.add(Cone { radius, height })),
            MeshMaterial3d(stalactite_mat.clone()),
            Transform::from_xyz(x, 0.0, z),
        ));
    }

    // 5 floor rocks: (x, z, radius)
    let floor_rocks: &[(f32, f32, f32)] = &[
        ( 3.2,  0.5, 0.30),
        (-3.4, -1.2, 0.40),
        ( 2.8, -2.5, 0.25),
        (-2.5,  2.8, 0.45),
        ( 3.5,  2.2, 0.20),
    ];

    for &(x, z, radius) in floor_rocks {
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("FloorRock"),
            Mesh3d(meshes.add(Sphere::new(radius))),
            MeshMaterial3d(floor_rock_mat.clone()),
            Transform::from_xyz(x, radius, z),
        ));
    }

    // Outer cave wall — large cylinder surrounding the entire play area
    commands.spawn((
        GameEntity,
        CaveObject,
        Name::new("CaveWall"),
        Mesh3d(meshes.add(Cylinder { radius: 8.5, half_height: 4.25 })),
        MeshMaterial3d(wall_mat),
        Transform::from_xyz(0.0, 4.25, 0.0),
    ));

    // 3 ember light sources: (x, z) — keep plain StandardMaterial for emissive
    let ember_positions: &[(f32, f32)] = &[
        ( 1.5,  1.8),
        (-2.2, -0.8),
        ( 2.5, -1.8),
    ];

    let ember_mat = std_materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.05),
        emissive: LinearRgba::rgb(2.0, 0.8, 0.1),
        ..default()
    });

    for (i, &(x, z)) in ember_positions.iter().enumerate() {
        let point_light = if i == 0 {
            PointLight {
                color: Color::srgb(1.0, 0.55, 0.1),
                intensity: 80_000.0,
                range: 8.0,
                shadows_enabled: true,
                shadow_depth_bias: 0.02,
                ..default()
            }
        } else {
            PointLight {
                color: Color::srgb(1.0, 0.55, 0.1),
                intensity: 60_000.0,
                range: 6.0,
                shadows_enabled: false,
                ..default()
            }
        };
        commands.spawn((
            GameEntity,
            CaveObject,
            Name::new("Ember"),
            Mesh3d(meshes.add(Sphere::new(0.06))),
            MeshMaterial3d(ember_mat.clone()),
            Transform::from_xyz(x, 0.3, z),
            Interactable { radius: 1.5 },
            Collider { radius: 0.12 },
            Lightable { lit: false },
        )).with_child((
            point_light,
            Transform::default(),
        ));
    }

    // Warm fill light — lifts shadows so crevices aren't pitch black
    commands.spawn((
        GameEntity,
        CaveObject,
        DirectionalLight {
            color: Color::srgb(0.30, 0.20, 0.10),
            illuminance: 120.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
}

pub struct CavePlugin;

impl Plugin for CavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), setup_cave);
    }
}
