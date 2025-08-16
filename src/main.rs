use area::setup_area;
use bevy::{
    prelude::{
        App, AppExtStates, ClearColor, Color, DefaultPlugins, IntoScheduleConfigs, PluginGroup,
        Resource, Update, Window, WindowPlugin, in_state,
    },
    state::state::OnEnter,
};

mod area;
mod camera;
mod characters;
mod loading;
mod state;
mod ui;

use camera::{orbit_camera, setup_camera};
use characters::{Cat, change_mode, move_cat, setup_cat};
use loading::LoadingPlugin;
use state::State;
use ui::{AreasMenuPlugin, MainMenuPlugin, OptionsPlugin};

#[derive(Resource, Default)]
struct Game {}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            LoadingPlugin,
            MainMenuPlugin,
            AreasMenuPlugin,
            OptionsPlugin,
        ))
        .init_resource::<Game>()
        .init_state::<State>()
        .enable_state_scoped_entities::<State>()
        .add_systems(
            OnEnter(State::Playing),
            (setup_area, setup_cat, setup_camera.after(setup_cat)),
        )
        .add_systems(
            Update,
            (orbit_camera, change_mode, move_cat).run_if(in_state(State::Playing)),
        );
    app.run();
}
