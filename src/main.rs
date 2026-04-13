use bevy::{
    prelude::{
        App, AppExtStates, ClearColor, Color, Commands, DefaultPlugins, Entity, IntoScheduleConfigs,
        PluginGroup, Query, Resource, Update, Window, WindowPlugin, With, in_state,
    },
    state::state::{OnEnter, OnExit},
    window::{CursorGrabMode, CursorOptions, WindowMode, PrimaryWindow},
};
use game::area::{AreaBounds, GameEntity, apply_area_bounds, setup_area};

mod audio;
mod characters;
mod game;
mod loading;
mod render;
mod settings;
mod state;
mod ui;

use characters::{CatPlugin, setup_cat};
use game::camera::{orbit_camera_keyboard, orbit_camera_mouse, setup_camera};
use game::{CavePlugin, CrystalCavernPlugin, InteractablesPlugin};
use game::area::SelectedArea;
use loading::LoadingPlugin;
use audio::AudioPlugin;
use render::{BlurPlugin, RockMaterialPlugin};
use settings::SettingsPlugin;
use state::State;
use ui::{AreasMenuPlugin, MainMenuPlugin, OptionsPlugin, PausedPlugin, common::despawn_menu_camera};


#[derive(Resource, Default)]
struct Game {}

fn set_cursor_hidden(mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursors.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn set_cursor_visible(mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursors.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn cleanup_game_world(query: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.063, 0.063, 0.114)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Stray Embers".to_string(),
                mode: WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            LoadingPlugin,
            SettingsPlugin,
            CatPlugin,
            MainMenuPlugin,
            AreasMenuPlugin,
            OptionsPlugin,
            PausedPlugin,
            AudioPlugin,
            BlurPlugin,
            RockMaterialPlugin,
            CavePlugin,
            CrystalCavernPlugin,
            InteractablesPlugin,
        ))
        .init_resource::<Game>()
        .init_resource::<SelectedArea>()
        .init_resource::<AreaBounds>()
        .init_state::<State>()
        .add_systems(OnEnter(State::MainMenu), cleanup_game_world)
        .add_systems(
            OnEnter(State::Playing),
            (
                apply_area_bounds,
                despawn_menu_camera,
                setup_area.after(apply_area_bounds),
                setup_camera.after(setup_cat),
                set_cursor_hidden,
            ),
        )
        .add_systems(OnExit(State::Playing), set_cursor_visible)
        .add_systems(
            Update,
            (orbit_camera_keyboard, orbit_camera_mouse).run_if(in_state(State::Playing)),
        );
    app.run();
}
