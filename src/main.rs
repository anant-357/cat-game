use area::setup_area;
use bevy::{
    app::PreStartup,
    prelude::{
        App, AppExtStates, Assets, Camera3d, Circle, ClearColor, Color, Commands, Cuboid,
        DefaultPlugins, Mesh, Mesh3d, MeshMaterial3d, Name, PluginGroup, PointLight, Quat, ResMut,
        Resource, StandardMaterial, Startup, StateScoped, Transform, Vec3, Window, WindowPlugin,
        default,
    },
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

mod area;
mod cat;
mod state;

use cat::{Cat, setup_cat};
use state::State;

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
        .init_resource::<Game>()
        .init_state::<State>()
        .enable_state_scoped_entities::<State>();
    app.add_systems(PreStartup, setup_camera)
        .add_systems(Startup, (setup_area, setup_cat));
    //app.add_systems(Update, move_cube);
    app.run();
}
