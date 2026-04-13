use std::str::FromStr;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    asset::AssetServer,
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        hierarchy::Children,
        name::Name,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    input_focus::{InputFocus, directional_navigation::DirectionalNavigationMap},
    math::CompassOctant,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, States},
        state_scoped::DespawnOnExit,
    },
    text::TextColor,
    ui::{Interaction, widget::{Button, Text}},
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{
    settings::AppSettings,
    state::State,
    ui::common::{spawn_camera, spawn_slider},
};

use super::common::{
    BG_DARK, PANEL_BG, TEXT_PRIMARY, button_text, get_button_bundle, highlight_focused_element,
    navigate, reset_button_after_interaction, spawn_divider, spawn_menu_root, spawn_panel,
    spawn_title,
};

#[derive(Component)]
pub struct Options;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Default,
    States,
    EnumIter,
    IntoStaticStr,
    EnumCount,
    EnumString,
)]
pub enum OptionsEnum {
    #[default]
    Fullscreen,
    #[strum(to_string = "Invert Mouse")]
    InvertMouse,
    Volume,
    #[strum(to_string = "Main Menu")]
    MainMenu,
}

fn label_for(option: OptionsEnum, settings: &AppSettings) -> String {
    match option {
        OptionsEnum::Fullscreen => format!(
            "Fullscreen: {}",
            if settings.fullscreen { "ON" } else { "OFF" }
        ),
        OptionsEnum::InvertMouse => format!(
            "Invert Mouse: {}",
            if settings.invert_mouse { "ON" } else { "OFF" }
        ),
        OptionsEnum::Volume => "Volume".to_string(),
        OptionsEnum::MainMenu => "Main Menu".to_string(),
    }
}

fn setup_ui(
    mut commands: Commands,
    settings: Res<AppSettings>,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    asset_server: Res<AssetServer>,
) {
    let title_font = asset_server.load("fonts/Cinzel-Regular.ttf");
    let body_font = asset_server.load("fonts/Nunito-Regular.ttf");

    let root = spawn_menu_root(&mut commands, State::OptionsMenu, BG_DARK);
    let panel = spawn_panel(&mut commands, State::OptionsMenu, PANEL_BG);
    commands.entity(root).add_child(panel);
    commands.entity(panel).insert(Options);

    let title = spawn_title(&mut commands, "Options", State::OptionsMenu, title_font);
    let divider = spawn_divider(&mut commands, State::OptionsMenu);
    commands.entity(panel).add_child(title);
    commands.entity(panel).add_child(divider);

    let mut button_entities: Vec<Entity> = Vec::new();
    for option in OptionsEnum::iter() {
        let name: &'static str = option.into();
        if option == OptionsEnum::Volume {
            let slider = spawn_slider(
                &mut commands,
                name.into(),
                settings.volume * 100.0,
                0.,
                100.,
            );
            commands.entity(panel).add_child(slider);
            button_entities.push(slider);
        } else {
            let label = label_for(option, &settings);
            let button = commands
                .spawn((
                    DespawnOnExit(State::OptionsMenu),
                    get_button_bundle(name.into()),
                ))
                .with_child(button_text(&label, body_font.clone()))
                .id();
            commands.entity(panel).add_child(button);
            button_entities.push(button);
        }
    }
    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    input_focus.set(button_entities[0]);
}

fn interact_with_focused_button(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    input_focus: Res<InputFocus>,
    buttons: Query<(Entity, &Name, &Interaction), With<Button>>,
    mut next_state: ResMut<NextState<State>>,
    mut settings: ResMut<AppSettings>,
) {
    let key_pressed = keyboard_input.just_pressed(KeyCode::Space)
        || keyboard_input.just_pressed(KeyCode::Enter);
    let mouse_clicked = mouse_input.just_pressed(MouseButton::Left);

    for (entity, name, interaction) in &buttons {
        let activated = (key_pressed && input_focus.0 == Some(entity))
            || (mouse_clicked && *interaction == Interaction::Pressed);
        if !activated {
            continue;
        }
        match OptionsEnum::from_str(name.as_str()) {
            Ok(OptionsEnum::Fullscreen) => {
                settings.fullscreen = !settings.fullscreen;
            }
            Ok(OptionsEnum::InvertMouse) => {
                settings.invert_mouse = !settings.invert_mouse;
            }
            Ok(OptionsEnum::MainMenu) => {
                next_state.set(State::MainMenu);
            }
            _ => (),
        }
    }
}

/// Keep toggle button labels in sync with the current settings values.
fn update_toggle_labels(
    settings: Res<AppSettings>,
    buttons: Query<(&Name, &Children)>,
    text_entities: Query<Entity, With<Text>>,
    mut commands: Commands,
) {
    if !settings.is_changed() {
        return;
    }
    for (name, children) in &buttons {
        let new_label = match OptionsEnum::from_str(name.as_str()) {
            Ok(opt @ (OptionsEnum::Fullscreen | OptionsEnum::InvertMouse)) => {
                Some(label_for(opt, &settings))
            }
            _ => None,
        };
        if let Some(label) = new_label {
            for &child in children.iter() {
                if text_entities.get(child).is_ok() {
                    commands.entity(child).insert((
                        Text::new(label.clone()),
                        TextColor(TEXT_PRIMARY),
                    ));
                }
            }
        }
    }
}

/// Sync the Volume slider value → AppSettings → GlobalVolume.
fn sync_volume_slider(
    sliders: Query<(&Name, &bevy::ui_widgets::SliderValue), bevy::ecs::query::Changed<bevy::ui_widgets::SliderValue>>,
    mut settings: ResMut<AppSettings>,
) {
    for (name, slider_value) in &sliders {
        if name.as_str() == "Volume" {
            settings.volume = (slider_value.0 / 100.0).clamp(0.0, 1.0);
        }
    }
}

pub struct OptionsPlugin;
impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            OnEnter(State::OptionsMenu),
            (spawn_camera, setup_ui.after(spawn_camera)),
        )
        .add_systems(PreUpdate, navigate.run_if(in_state(State::OptionsMenu)))
        .add_systems(
            Update,
            (
                highlight_focused_element,
                interact_with_focused_button,
                reset_button_after_interaction,
                update_toggle_labels,
                sync_volume_slider,
            )
                .run_if(in_state(State::OptionsMenu)),
        );
    }
}
