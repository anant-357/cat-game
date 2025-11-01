use std::str::FromStr;

use bevy::{
    app::{AppExit, Plugin, PreUpdate, Update},
    ecs::{
        entity::Entity,
        message::MessageWriter,
        name::Name,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
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
    ui::{Display, Node, RepeatedGridTrack, Val, widget::Text},
    utils::default,
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{game::camera::setup_camera, state::State, ui::common::spawn_camera};

use super::common::{
    get_button_bundle, highlight_focused_element, navigate, reset_button_after_interaction,
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
    ChooseArea,
    Options,
    Exit,
}

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    let root_node = commands
        .spawn((
            DespawnOnExit(State::MainMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .id();

    let grid_root_entity = commands
        .spawn((
            DespawnOnExit(State::MainMenu),
            Node {
                display: Display::Grid,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                grid_template_columns: RepeatedGridTrack::auto(1),
                grid_template_rows: RepeatedGridTrack::auto(3),
                ..default()
            },
        ))
        .id();

    commands.entity(root_node).add_child(grid_root_entity);

    let mut button_entities: Vec<Entity> = Vec::new();
    for option in MainMenuEnum::iter() {
        let name: &'static str = option.into();
        let button_entity = commands
            .spawn((
                DespawnOnExit(State::MainMenu),
                get_button_bundle(name.into()),
            ))
            .with_child(Text::new(name.to_string()))
            .id();
        commands.entity(grid_root_entity).add_child(button_entity);
        button_entities.push(button_entity);
    }

    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    let top = button_entities[0];
    input_focus.set(top);
}

fn interact_with_focused_button(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    query: Query<(Entity, &Name), With<Name>>,
    mut exit: MessageWriter<AppExit>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(focused_entity) = input_focus.0 {
            for (e, name) in query.iter() {
                if focused_entity == e {
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
