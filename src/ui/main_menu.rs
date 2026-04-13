use std::str::FromStr;

use bevy::{
    app::{AppExit, Plugin, PreUpdate, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        message::MessageWriter,
        name::Name,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    input_focus::{
        InputDispatchPlugin, InputFocus, InputFocusVisible,
        directional_navigation::{DirectionalNavigationMap, DirectionalNavigationPlugin},
    },
    math::CompassOctant,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, States},
        state_scoped::DespawnOnExit,
    },
    ui::{Interaction, widget::Button},
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{state::State, ui::common::spawn_camera};

use super::common::{
    BG_DARK, PANEL_BG, button_text, get_button_bundle, highlight_focused_element, navigate,
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
    States,
    EnumIter,
    IntoStaticStr,
    EnumCount,
    EnumString,
)]
pub enum MainMenuEnum {
    #[default]
    Play,
    #[strum(to_string = "Choose Area")]
    ChooseArea,
    Options,
    Exit,
}

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    asset_server: Res<AssetServer>,
) {
    let title_font = asset_server.load("fonts/Cinzel-Regular.ttf");
    let body_font = asset_server.load("fonts/Nunito-Regular.ttf");

    let root = spawn_menu_root(&mut commands, State::MainMenu, BG_DARK);
    let panel = spawn_panel(&mut commands, State::MainMenu, PANEL_BG);
    commands.entity(root).add_child(panel);

    let title = spawn_title(&mut commands, "Stray Embers", State::MainMenu, title_font);
    let divider = spawn_divider(&mut commands, State::MainMenu);
    commands.entity(panel).add_child(title);
    commands.entity(panel).add_child(divider);

    let mut button_entities: Vec<Entity> = Vec::new();
    for option in MainMenuEnum::iter() {
        let name: &'static str = option.into();
        let button_entity = commands
            .spawn((
                DespawnOnExit(State::MainMenu),
                get_button_bundle(name.into()),
            ))
            .with_child(button_text(name, body_font.clone()))
            .id();
        commands.entity(panel).add_child(button_entity);
        button_entities.push(button_entity);
    }

    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    input_focus.set(button_entities[0]);
}

fn interact_with_focused_button(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    input_focus: Res<InputFocus>,
    buttons: Query<(Entity, &Name, &Interaction), With<Button>>,
    mut exit: MessageWriter<AppExit>,
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
        match MainMenuEnum::from_str(name.as_str()) {
            Ok(MainMenuEnum::Play) => {
                next_state.set(State::Playing);
            }
            Ok(MainMenuEnum::ChooseArea) => {
                next_state.set(State::ChooseArea);
            }
            Ok(MainMenuEnum::Options) => {
                next_state.set(State::OptionsMenu);
            }
            Ok(MainMenuEnum::Exit) => {
                exit.write(AppExit::Success);
            }
            _ => (),
        }
    }
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin))
            .insert_resource(InputFocusVisible(true))
            .add_systems(
                OnEnter(State::MainMenu),
                (spawn_camera, setup_ui.after(spawn_camera)),
            )
            .add_systems(PreUpdate, navigate.run_if(in_state(State::MainMenu)))
            .add_systems(
                Update,
                (
                    highlight_focused_element,
                    interact_with_focused_button,
                    reset_button_after_interaction,
                )
                    .run_if(in_state(State::MainMenu)),
            );
    }
}
