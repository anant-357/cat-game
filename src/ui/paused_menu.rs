use std::str::FromStr;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        entity::Entity,
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
        state::{NextState, OnEnter},
        state_scoped::DespawnOnExit,
    },
    ui::{Interaction, widget::Button},
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::state::State;

use super::common::{
    button_text, get_button_bundle, highlight_focused_element, navigate,
    reset_button_after_interaction, spawn_divider, spawn_menu_root, spawn_panel, spawn_title,
};

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Default,
    EnumIter,
    IntoStaticStr,
    EnumCount,
    EnumString,
)]
pub enum PausedMenuEnum {
    #[default]
    Resume,
    #[strum(to_string = "Main Menu")]
    MainMenu,
}

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    asset_server: Res<AssetServer>,
) {
    let title_font = asset_server.load("fonts/Cinzel-Regular.ttf");
    let body_font = asset_server.load("fonts/Nunito-Regular.ttf");

    // Semi-transparent overlay so the blurred game shows through
    let root = spawn_menu_root(
        &mut commands,
        State::Paused,
        Color::srgba(0.02, 0.02, 0.05, 0.75),
    );
    let panel = spawn_panel(
        &mut commands,
        State::Paused,
        Color::srgba(0.04, 0.04, 0.09, 0.85),
    );
    commands.entity(root).add_child(panel);

    let title = spawn_title(&mut commands, "Paused", State::Paused, title_font);
    let divider = spawn_divider(&mut commands, State::Paused);
    commands.entity(panel).add_child(title);
    commands.entity(panel).add_child(divider);

    let mut button_entities = Vec::new();
    for item in PausedMenuEnum::iter() {
        let name: &'static str = item.into();
        let button = commands
            .spawn((
                DespawnOnExit(State::Paused),
                get_button_bundle(name.to_string()),
            ))
            .with_child(button_text(name, body_font.clone()))
            .id();
        commands.entity(panel).add_child(button);
        button_entities.push(button);
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
        match PausedMenuEnum::from_str(name.as_str()) {
            Ok(PausedMenuEnum::Resume) => {
                next_state.set(State::Playing);
            }
            Ok(PausedMenuEnum::MainMenu) => {
                next_state.set(State::MainMenu);
            }
            _ => (),
        }
    }
}

fn resume_on_escape(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(State::Playing);
    }
}

pub struct PausedPlugin;
impl Plugin for PausedPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(State::Paused), setup_ui)
            .add_systems(PreUpdate, navigate.run_if(in_state(State::Paused)))
            .add_systems(
                Update,
                (
                    highlight_focused_element,
                    interact_with_focused_button,
                    reset_button_after_interaction,
                    resume_on_escape,
                )
                    .run_if(in_state(State::Paused)),
            );
    }
}
