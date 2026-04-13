use bevy::prelude::*;

use crate::{
    characters::CatLocomotion,
    game::interactables::{EmberProgress, Highlighted, EMBER_TOTAL},
    state::State,
};

// ─── Resources ───────────────────────────────────────────────────────────────

#[derive(Resource)]
struct AudioHandles {
    footstep:    Handle<AudioSource>,
    interact:    Handle<AudioSource>,
    ember_light: Handle<AudioSource>,
    win:         Handle<AudioSource>,
}

#[derive(Resource)]
struct FootstepTimer {
    timer: Timer,
    was_moving: bool,
}

impl Default for FootstepTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.38, TimerMode::Once),
            was_moving: false,
        }
    }
}

#[derive(Resource, Default)]
struct WinPlayed(bool);

// ─── Setup ───────────────────────────────────────────────────────────────────

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Ambient drone — looping
    let ambient: Handle<AudioSource> = asset_server.load("audio/ambient.wav");
    commands.spawn((AudioPlayer(ambient), PlaybackSettings::LOOP));

    commands.insert_resource(AudioHandles {
        footstep:    asset_server.load("audio/footstep.wav"),
        interact:    asset_server.load("audio/interact.wav"),
        ember_light: asset_server.load("audio/ember_light.wav"),
        win:         asset_server.load("audio/win.wav"),
    });
    commands.insert_resource(FootstepTimer::default());
    commands.insert_resource(WinPlayed(false));
}

// ─── Systems ─────────────────────────────────────────────────────────────────

fn play_footstep(
    mut commands: Commands,
    time: Res<Time>,
    mut ft: ResMut<FootstepTimer>,
    handles: Res<AudioHandles>,
    cat_query: Query<&CatLocomotion>,
) {
    let Ok(loco) = cat_query.single() else { return };
    let moving = loco.velocity > 0.0;

    ft.timer.tick(time.delta());

    if moving && ft.timer.just_finished() {
        // Shorter interval when running
        let interval = if loco.velocity >= 0.15 { 0.22 } else { 0.38 };
        ft.timer = Timer::from_seconds(interval, TimerMode::Once);
        commands.spawn((
            AudioPlayer(handles.footstep.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    // When stopping, wind the timer forward so the next step fires immediately on resume
    if ft.was_moving && !moving {
        let dur = ft.timer.duration();
        ft.timer = Timer::from_seconds(dur.as_secs_f32(), TimerMode::Once);
        ft.timer.tick(dur);
    }
    ft.was_moving = moving;
}

fn play_interact_sfx(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    handles: Res<AudioHandles>,
    highlighted: Query<(), With<Highlighted>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) && !highlighted.is_empty() {
        commands.spawn((
            AudioPlayer(handles.interact.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn play_ember_sfx(
    mut commands: Commands,
    progress: Res<EmberProgress>,
    handles: Res<AudioHandles>,
    mut win_played: ResMut<WinPlayed>,
) {
    if !progress.is_changed() { return; }

    if progress.lit > 0 {
        commands.spawn((
            AudioPlayer(handles.ember_light.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    if progress.lit >= EMBER_TOTAL && !win_played.0 {
        win_played.0 = true;
        commands.spawn((
            AudioPlayer(handles.win.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Playing), setup_audio)
            .add_systems(
                Update,
                (play_footstep, play_interact_sfx, play_ember_sfx)
                    .run_if(in_state(State::Playing)),
            );
    }
}
