use bevy::{
    color::LinearRgba,
    prelude::*,
    text::{TextColor, TextFont},
    ui::{AlignItems, FlexDirection, JustifyContent, Node, PositionType, UiRect, Val, widget::Text},
};

use crate::{characters::Cat, game::area::{AreaBounds, GameEntity, SelectedArea, Area}, state::State, ui::common::TEXT_PRIMARY};

// ─── Components ──────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct Interactable {
    pub radius: f32,
}

/// Slides away from the cat when interacted with.
#[derive(Component)]
pub struct Pushable;

/// Rotates 90° on the first interaction, then stays tipped.
#[derive(Component)]
pub struct Tippable {
    pub tipped: bool,
}

/// Can be lit by the cat pressing E — used for cave embers.
#[derive(Component)]
pub struct Lightable {
    pub lit: bool,
}

/// Solid XZ collider — prevents the cat from walking through.
/// `radius` is the XZ circle radius used for collision resolution.
#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

/// Tag added to the nearest in-range interactable each frame.
#[derive(Component)]
pub struct Highlighted;

/// Tag on the "Press E to interact" UI text entity.
#[derive(Component)]
pub struct InteractPrompt;

/// Tag on the ember progress HUD text.
#[derive(Component)]
struct EmberHud;

/// Tag on the crystal progress HUD text.
#[derive(Component)]
struct CrystalHud;

/// Tag on the "Cave cleared!" / "Cavern awakened!" win banner.
#[derive(Component)]
struct WinBanner;

/// Marks a crystal node interactable in the Crystal Cavern.
#[derive(Component)]
pub struct CrystalNode;

// ─── Resources ───────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct EmberProgress {
    pub lit: u32,
}

pub const EMBER_TOTAL: u32 = 3;

#[derive(Resource, Default)]
pub struct CrystalProgress {
    pub activated: u32,
}

pub const CRYSTAL_TOTAL: u32 = 4;

// ─── Setup ───────────────────────────────────────────────────────────────────

pub fn setup_interactables(
    existing: Query<(), With<Interactable>>,
    selected: Res<SelectedArea>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if !existing.is_empty() {
        return; // already spawned (guard against re-entry on resume)
    }

    // Ball — can be pushed around the arena
    commands.spawn((
        GameEntity,
        Name::new("Ball"),
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.1),
            ..default()
        })),
        Transform::from_xyz(2.0, 0.5, 0.5),
        Interactable { radius: 1.5 },
        Collider { radius: 0.5 },
        Pushable,
    ));

    // Crate — can be tipped over once
    commands.spawn((
        GameEntity,
        Name::new("Crate"),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.4, 0.2),
            ..default()
        })),
        Transform::from_xyz(-1.5, 0.5, -2.0),
        Interactable { radius: 1.5 },
        Collider { radius: 0.72 },
        Tippable { tipped: false },
    ));

    let body_font = asset_server.load("fonts/Nunito-Regular.ttf");

    // Full-screen transparent HUD overlay — holds the interact prompt at the bottom
    commands
        .spawn((
            GameEntity,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
        ))
        .with_child((
            InteractPrompt,
            Text::new("Press E to interact"),
            TextFont { font: body_font.clone(), font_size: 18.0, ..default() },
            TextColor(TEXT_PRIMARY),
            Visibility::Hidden,
        ));

    // Progress HUD — top-right corner (area-specific)
    let hud_container = commands.spawn((
        GameEntity,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::End,
            padding: UiRect::axes(Val::Px(24.0), Val::Px(20.0)),
            ..default()
        },
    )).id();

    match selected.0 {
        Area::Cave => {
            commands.entity(hud_container).with_child((
                EmberHud,
                Text::new("Embers: 0/3"),
                TextFont { font: body_font.clone(), font_size: 18.0, ..default() },
                TextColor(TEXT_PRIMARY),
            ));
        }
        Area::CrystalCavern => {
            commands.entity(hud_container).with_child((
                CrystalHud,
                Text::new("Crystals: 0/4"),
                TextFont { font: body_font.clone(), font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.7, 0.5, 1.0)),
            ));
        }
    }

    // Centered win banner — hidden until objective is complete
    let win_text = match selected.0 {
        Area::Cave          => "Cave cleared!",
        Area::CrystalCavern => "Cavern awakened!",
    };
    commands.spawn((
        GameEntity,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ))
    .with_child((
        WinBanner,
        Text::new(win_text),
        TextFont { font: body_font, font_size: 48.0, ..default() },
        TextColor(Color::srgb(0.98, 0.85, 0.30)),
        Visibility::Hidden,
    ));
}

fn reset_progress(
    mut ember: ResMut<EmberProgress>,
    mut crystal: ResMut<CrystalProgress>,
) {
    ember.lit = 0;
    crystal.activated = 0;
}

// ─── Systems ─────────────────────────────────────────────────────────────────

/// Each frame: find the nearest in-range interactable, tag it `Highlighted`,
/// apply an amber emissive glow, and show/hide the prompt text.
pub fn update_highlights(
    mut commands: Commands,
    cat_query: Query<&Transform, With<Cat>>,
    interactables: Query<(Entity, &Interactable, &Transform, &MeshMaterial3d<StandardMaterial>, Option<&Lightable>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut prompts: Query<&mut Visibility, With<InteractPrompt>>,
) {
    let cat_pos = match cat_query.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    // Find the nearest unlocked interactable within its interaction radius
    let mut nearest: Option<Entity> = None;
    let mut nearest_dist = f32::MAX;
    for (entity, interactable, transform, _, lightable) in &interactables {
        // Skip already-lit embers — they're done
        if lightable.map(|l| l.lit).unwrap_or(false) { continue; }
        let dist = (transform.translation - cat_pos).xz().length();
        if dist < interactable.radius && dist < nearest_dist {
            nearest_dist = dist;
            nearest = Some(entity);
        }
    }

    // Add/remove Highlighted tag and update emissive glow
    for (entity, _, _, mat_handle, lightable) in &interactables {
        // Never dim a lit ember
        if lightable.map(|l| l.lit).unwrap_or(false) { continue; }
        if nearest == Some(entity) {
            commands.entity(entity).insert(Highlighted);
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.emissive = LinearRgba::rgb(0.6, 0.35, 0.0);
            }
        } else {
            commands.entity(entity).remove::<Highlighted>();
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.emissive = LinearRgba::BLACK;
            }
        }
    }

    // Show or hide the "Press E" prompt
    let visibility = if nearest.is_some() { Visibility::Visible } else { Visibility::Hidden };
    for mut vis in &mut prompts {
        *vis = visibility;
    }
}

/// On E press: push the highlighted ball, tip the highlighted crate, or light the highlighted ember/crystal.
pub fn handle_interact(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    bounds: Res<AreaBounds>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ember_progress: ResMut<EmberProgress>,
    mut crystal_progress: ResMut<CrystalProgress>,
    crystal_nodes: Query<(), With<CrystalNode>>,
    mut param_set: ParamSet<(
        Query<&Transform, With<Cat>>,
        Query<(
            Entity,
            &mut Transform,
            Option<&Pushable>,
            Option<&mut Tippable>,
            Option<&mut Lightable>,
            &MeshMaterial3d<StandardMaterial>,
        ), With<Highlighted>>,
    )>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    let cat_pos = {
        let q = param_set.p0();
        match q.single() {
            Ok(t) => t.translation,
            Err(_) => return,
        }
    };

    for (entity, mut transform, pushable, tippable, lightable, mat_handle) in param_set.p1().iter_mut() {
        if pushable.is_some() {
            let obj_pos = transform.translation;
            let dir = (obj_pos - cat_pos).with_y(0.0).normalize_or_zero();
            let raw = Vec2::new(obj_pos.x + dir.x * 1.5, obj_pos.z + dir.z * 1.5);
            let new_xz = if raw.length() > bounds.play_radius { raw.normalize() * bounds.play_radius } else { raw };
            transform.translation.x = new_xz.x;
            transform.translation.z = new_xz.y;
        }
        if let Some(mut tip) = tippable {
            if !tip.tipped {
                transform.rotate_z(std::f32::consts::FRAC_PI_2);
                tip.tipped = true;
            }
        }
        if let Some(mut lightable) = lightable {
            if !lightable.lit {
                lightable.lit = true;
                let is_crystal = crystal_nodes.contains(entity);
                if let Some(mat) = materials.get_mut(&mat_handle.0) {
                    if is_crystal {
                        mat.emissive = LinearRgba::rgb(1.5, 0.5, 4.0);
                        mat.base_color = Color::srgb(0.7, 0.4, 1.0);
                    } else {
                        mat.emissive = LinearRgba::rgb(6.0, 2.5, 0.2);
                        mat.base_color = Color::srgb(1.0, 0.75, 0.2);
                    }
                }
                if is_crystal {
                    crystal_progress.activated += 1;
                } else {
                    ember_progress.lit += 1;
                }
            }
        }
    }
}

/// Updates the ember HUD text and shows the win banner when all embers are lit.
fn update_ember_hud(
    progress: Res<EmberProgress>,
    mut hud: Query<&mut Text, With<EmberHud>>,
    mut banners: Query<&mut Visibility, With<WinBanner>>,
) {
    if !progress.is_changed() { return; }
    for mut text in &mut hud {
        **text = format!("Embers: {}/{}", progress.lit, EMBER_TOTAL);
    }
    let won = progress.lit >= EMBER_TOTAL;
    for mut vis in &mut banners {
        *vis = if won { Visibility::Visible } else { Visibility::Hidden };
    }
}

/// Updates the crystal HUD text and shows the win banner when all crystal nodes are activated.
fn update_crystal_hud(
    progress: Res<CrystalProgress>,
    mut hud: Query<&mut Text, With<CrystalHud>>,
    mut banners: Query<&mut Visibility, With<WinBanner>>,
) {
    if !progress.is_changed() { return; }
    for mut text in &mut hud {
        **text = format!("Crystals: {}/{}", progress.activated, CRYSTAL_TOTAL);
    }
    let won = progress.activated >= CRYSTAL_TOTAL;
    for mut vis in &mut banners {
        *vis = if won { Visibility::Visible } else { Visibility::Hidden };
    }
}

// ─── Collision resolution ─────────────────────────────────────────────────────

const CAT_RADIUS: f32 = 0.30;

/// Pushes the cat out of any overlapping `Collider` entities each frame.
/// Pushable objects share the displacement; static objects push the cat fully.
pub fn resolve_collisions(
    bounds: Res<AreaBounds>,
    mut cats: Query<&mut Transform, With<Cat>>,
    mut colliders: Query<(&mut Transform, &Collider, Option<&Pushable>), Without<Cat>>,
) {
    let Ok(mut cat_tf) = cats.single_mut() else { return; };

    for (mut obj_tf, collider, pushable) in &mut colliders {
        let diff = Vec2::new(
            cat_tf.translation.x - obj_tf.translation.x,
            cat_tf.translation.z - obj_tf.translation.z,
        );
        let dist = diff.length();
        let min_dist = CAT_RADIUS + collider.radius;

        if dist >= min_dist || dist < 1e-4 { continue; }

        let push = diff / dist * (min_dist - dist);

        if pushable.is_some() {
            // Split displacement 50/50 so the ball rolls away naturally
            cat_tf.translation.x += push.x * 0.5;
            cat_tf.translation.z += push.y * 0.5;
            obj_tf.translation.x -= push.x * 0.5;
            obj_tf.translation.z -= push.y * 0.5;

            let obj_xz = Vec2::new(obj_tf.translation.x, obj_tf.translation.z);
            if obj_xz.length() > bounds.play_radius {
                let c = obj_xz.normalize() * bounds.play_radius;
                obj_tf.translation.x = c.x;
                obj_tf.translation.z = c.y;
            }
        } else {
            // Static — push cat fully out
            cat_tf.translation.x += push.x;
            cat_tf.translation.z += push.y;
        }

        // Re-clamp cat after any push
        let cat_xz = Vec2::new(cat_tf.translation.x, cat_tf.translation.z);
        if cat_xz.length() > bounds.play_radius {
            let c = cat_xz.normalize() * bounds.play_radius;
            cat_tf.translation.x = c.x;
            cat_tf.translation.z = c.y;
        }
    }
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmberProgress>()
            .init_resource::<CrystalProgress>()
            .add_systems(OnEnter(State::Playing), (setup_interactables, reset_progress))
            .add_systems(
                Update,
                (update_highlights, handle_interact, update_ember_hud, update_crystal_hud, resolve_collisions)
                    .run_if(in_state(State::Playing)),
            );
    }
}
