use area::setup_area;
use bevy::{
    prelude::{
        App, AppExtStates, Camera3d, ClearColor, Color, Commands, DefaultPlugins, PluginGroup,
        PointLight, Resource, Transform, Vec3, Window, WindowPlugin, default,
    },
    state::state::OnEnter,
};

mod area;
mod characters;
mod loading;
mod state;
mod ui;

use characters::{Cat, setup_cat};
use loading::{LoadingPlugin, LoadingState};
use state::State;
use ui::{AreasMenuPlugin, MainMenuPlugin, OptionsPlugin};

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Resource, Default)]
struct Game {
    pub loading_state: LoadingState,
    pub cat: Cat,
}

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
            (setup_camera, setup_area, setup_cat),
        );
    //app.add_systems(Update, move_cube);
    app.run();
}
