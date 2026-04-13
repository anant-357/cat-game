use bevy::{gltf::GltfAssetLabel, prelude::*};

use crate::{
    game::{area::{AreaBounds, GameEntity}, camera::CameraRig},
    state::State,
};

// ── Cat component ─────────────────────────────────────────────────────────────

#[derive(Default, Component)]
pub struct Cat {
    mode: CatMode,
}

#[derive(Default)]
enum CatMode {
    #[default]
    Normal,
    Black,
    White,
}

// ── Movement component ────────────────────────────────────────────────────────

/// Current movement speed — written by move_cat.
#[derive(Component, Default)]
pub struct CatLocomotion {
    pub velocity: f32,
    pub y_velocity: f32,
}

// ── Animation components ──────────────────────────────────────────────────────

#[derive(Component)]
pub struct CatAnimationNodes {
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run:  AnimationNodeIndex,
}

#[derive(Component, Default, PartialEq, Clone, Copy)]
enum CatAnimState {
    #[default]
    Idle,
    Walk,
    Run,
}

/// Holds the entity that owns the `AnimationPlayer` (a GLTF child entity).
#[derive(Component)]
pub struct CatAnimPlayer(pub Entity);

/// Marker — added once the animation graph has been wired up.
#[derive(Component)]
struct CatAnimationInitialized;

/// Stores handles to all StandardMaterial instances in the cat GLTF hierarchy.
#[derive(Component, Default)]
struct CatMeshMaterials(Vec<Handle<StandardMaterial>>);

// ── Setup ─────────────────────────────────────────────────────────────────────

pub fn setup_cat(
    existing: Query<(), With<Cat>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !existing.is_empty() {
        return;
    }
    commands.spawn((
        GameEntity,
        Name::new("Cat"),
        Transform::from_xyz(-1.0, 0.0, 0.0),
        Cat::default(),
        CatLocomotion::default(),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cat.glb"))),
    ));
}

// ── Animation init ────────────────────────────────────────────────────────────

fn collect_std_materials(
    root: Entity,
    children: &Query<&Children>,
    mat_handles: &Query<&MeshMaterial3d<StandardMaterial>>,
    out: &mut Vec<Handle<StandardMaterial>>,
) {
    if let Ok(h) = mat_handles.get(root) {
        out.push(h.0.clone());
    }
    if let Ok(kids) = children.get(root) {
        for child in kids.iter() {
            collect_std_materials(child, children, mat_handles, out);
        }
    }
}

fn find_animation_player(
    root: Entity,
    children: &Query<&Children>,
    players: &Query<Entity, With<AnimationPlayer>>,
) -> Option<Entity> {
    if players.contains(root) {
        return Some(root);
    }
    if let Ok(kids) = children.get(root) {
        for child in kids.iter() {
            if let Some(found) = find_animation_player(child, children, players) {
                return Some(found);
            }
        }
    }
    None
}

fn init_cat_animation(
    mut commands: Commands,
    cats: Query<Entity, (With<Cat>, Without<CatAnimationInitialized>)>,
    children: Query<&Children>,
    animation_players: Query<Entity, With<AnimationPlayer>>,
    mat_handles: Query<&MeshMaterial3d<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    for cat_entity in &cats {
        let Some(player_entity) =
            find_animation_player(cat_entity, &children, &animation_players)
        else {
            continue; // GLTF scene not fully loaded yet — retry next frame
        };

        // NLA strip order: 0=Idle, 1=Walk, 2=Run (matches _push_to_nla call order)
        let idle_clip: Handle<AnimationClip> =
            asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/cat.glb"));
        let walk_clip: Handle<AnimationClip> =
            asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/cat.glb"));
        let run_clip: Handle<AnimationClip> =
            asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/cat.glb"));

        let mut graph = AnimationGraph::new();
        let idle_node = graph.add_clip(idle_clip, 1.0, graph.root);
        let walk_node = graph.add_clip(walk_clip, 1.0, graph.root);
        let run_node  = graph.add_clip(run_clip,  1.0, graph.root);

        let graph_handle = graphs.add(graph);

        let mut mesh_materials = Vec::new();
        collect_std_materials(cat_entity, &children, &mat_handles, &mut mesh_materials);

        commands
            .entity(player_entity)
            .insert(AnimationGraphHandle(graph_handle));
        commands.entity(cat_entity).insert((
            CatAnimationNodes { idle: idle_node, walk: walk_node, run: run_node },
            CatAnimState::Idle,
            CatAnimPlayer(player_entity),
            CatAnimationInitialized,
            CatMeshMaterials(mesh_materials),
        ));
    }
}

// ── Animation update ──────────────────────────────────────────────────────────

fn animate_cat(
    mut players: Query<&mut AnimationPlayer>,
    mut cats: Query<
        (&CatLocomotion, &CatAnimationNodes, &CatAnimPlayer, &mut CatAnimState),
        With<CatAnimationInitialized>,
    >,
) {
    for (locomotion, nodes, cat_player, mut anim_state) in &mut cats {
        let Ok(mut player) = players.get_mut(cat_player.0) else {
            continue;
        };

        let target = if locomotion.velocity >= 0.15 {
            CatAnimState::Run
        } else if locomotion.velocity > 0.0 {
            CatAnimState::Walk
        } else {
            CatAnimState::Idle
        };

        if *anim_state != target {
            *anim_state = target;
            let node = match target {
                CatAnimState::Idle => nodes.idle,
                CatAnimState::Walk => nodes.walk,
                CatAnimState::Run  => nodes.run,
            };
            player.play(node).repeat();
        }
    }
}

// ── Movement ──────────────────────────────────────────────────────────────────

const GRAVITY: f32 = -22.0;
const JUMP_FORCE: f32 = 9.0;

fn move_cat(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    bounds: Res<AreaBounds>,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut CatLocomotion), With<Cat>>,
        Query<&Transform, With<CameraRig>>,
    )>,
) {
    let rig = set.p1().single().expect("Camera Rig not spawned").clone();
    let mut query = set.p0();
    let dt = time.delta_secs();

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

    for (mut transform, mut locomotion) in query.iter_mut() {
        // ── Jump & gravity ─────────────────────────────────────────────────
        let grounded = transform.translation.y <= 0.001;
        if keyboard_input.just_pressed(KeyCode::Space) && grounded {
            locomotion.y_velocity = JUMP_FORCE;
        }
        locomotion.y_velocity += GRAVITY * dt;
        transform.translation.y += locomotion.y_velocity * dt;
        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            locomotion.y_velocity = 0.0;
        }

        // ── Horizontal movement ────────────────────────────────────────────
        if input_direction != Vec3::ZERO {
            let forward = rig.forward().with_y(0.).normalize();
            let right   = rig.right().normalize();
            let move_direction =
                (input_direction.z * forward + input_direction.x * right).normalize();

            let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
                0.2 // run
            } else {
                0.1 // walk
            };

            transform.translation += move_direction * speed;

            // Clamp XZ to area play radius
            let xz = Vec2::new(transform.translation.x, transform.translation.z);
            if xz.length() > bounds.play_radius {
                let clamped = xz.normalize() * bounds.play_radius;
                transform.translation.x = clamped.x;
                transform.translation.z = clamped.y;
            }

            // Face the movement direction
            let target = transform.translation + move_direction;
            transform.look_at(target, Vec3::Y);

            locomotion.velocity = speed;
        } else {
            locomotion.velocity = 0.0;
        }
    }
}

// ── Color toggle ──────────────────────────────────────────────────────────────

fn change_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cats: Query<(&mut Cat, &CatMeshMaterials)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Tab) {
        return;
    }
    for (mut cat, mesh_mats) in &mut cats {
        cat.mode = match cat.mode {
            CatMode::Normal => CatMode::Black,
            CatMode::Black  => CatMode::White,
            CatMode::White  => CatMode::Normal,
        };
        let color = match cat.mode {
            CatMode::Normal => Color::srgb(0.478, 0.392, 0.082),
            CatMode::Black  => Color::srgb(0.05, 0.04, 0.04),
            CatMode::White  => Color::srgb(0.92, 0.92, 0.90),
        };
        for handle in &mesh_mats.0 {
            if let Some(mat) = materials.get_mut(handle) {
                mat.base_color = color;
            }
        }
    }
}

// ── Pause ─────────────────────────────────────────────────────────────────────

fn exit_play(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(State::Paused);
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct CatPlugin;
impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), setup_cat)
            .add_systems(
                Update,
                (
                    init_cat_animation,
                    move_cat,
                    animate_cat.after(move_cat),
                    change_mode,
                    exit_play,
                )
                    .run_if(in_state(State::Playing)),
            );
    }
}
