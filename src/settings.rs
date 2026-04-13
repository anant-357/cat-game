use bevy::{
    audio::{GlobalVolume, Volume},
    prelude::*,
    window::{MonitorSelection, WindowMode},
};
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.ron";

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    /// Master volume, 0.0 (silent) to 1.0 (full).
    pub volume: f32,
    pub fullscreen: bool,
    pub invert_mouse: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            volume: 0.5,
            fullscreen: true,
            invert_mouse: false,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        std::fs::read_to_string(SETTINGS_PATH)
            .ok()
            .and_then(|s| ron::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(s) = ron::to_string(self) {
            let _ = std::fs::write(SETTINGS_PATH, s);
        }
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(AppSettings::load());
}

fn apply_volume(settings: Res<AppSettings>, mut global_volume: ResMut<GlobalVolume>) {
    if settings.is_changed() {
        global_volume.volume = Volume::Linear(settings.volume);
    }
}

fn apply_fullscreen(settings: Res<AppSettings>, mut windows: Query<&mut Window>) {
    if settings.is_changed() {
        for mut window in &mut windows {
            window.mode = if settings.fullscreen {
                WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            } else {
                WindowMode::Windowed
            };
        }
    }
}

fn save_on_change(settings: Res<AppSettings>) {
    if settings.is_changed() {
        settings.save();
    }
}

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, (apply_volume, apply_fullscreen, save_on_change));
    }
}
