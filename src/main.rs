use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::Fullscreen(
                MonitorSelection::Primary,
                VideoModeSelection::Current,
            ),
            title: "Minesweeper".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    #[cfg(feature = "debug")]
    app.add_plugins((
        EguiPlugin {
            enable_multipass_for_primary_context: true,
        },
        WorldInspectorPlugin::new(),
    ));
    app.run();
}
