use std::str::FromStr;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        name::Name,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{InputFocus, directional_navigation::DirectionalNavigationMap},
    math::CompassOctant,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, OnExit, States},
    },
    ui::{Display, Node, RepeatedGridTrack, Val, widget::Text},
    utils::default,
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::state::State;

use super::common::{
    get_button_bundle, highlight_focused_element, navigate, reset_button_after_interaction,
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
    Option_1,
    Option_2,
    Main_Menu,
}

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands.spawn(Camera2d).insert(Options);

    let root_node = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(Options)
        .id();

    let grid_root_entity = commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_template_columns: RepeatedGridTrack::auto(1),
            grid_template_rows: RepeatedGridTrack::auto(3),
            ..default()
        })
        .id();

    commands.entity(root_node).add_child(grid_root_entity);

    let mut button_entities: Vec<Entity> = Vec::new();
    for option in OptionsEnum::iter() {
        let name: &'static str = option.into();
        let button = commands
            .spawn(get_button_bundle(name.into()))
            .with_child(Text::new(name.to_string()))
            .id();
        commands.entity(grid_root_entity).add_child(button);
        button_entities.push(button);
    }
    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    let top = button_entities[0];
    input_focus.set(top);
}

fn interact_with_focused_button(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    query: Query<(Entity, &Name), With<Name>>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(focused_entity) = input_focus.0 {
            for (e, name) in query.iter() {
                if focused_entity == e {
                    match OptionsEnum::from_str(name.as_str()) {
                        Ok(OptionsEnum::Main_Menu) => {
                            next_state.set(State::MainMenu);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<Options>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub struct OptionsPlugin;
impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(State::OptionsMenu), setup_ui)
            .add_systems(PreUpdate, navigate.run_if(in_state(State::OptionsMenu)))
            .add_systems(
                Update,
                (
                    highlight_focused_element,
                    interact_with_focused_button,
                    reset_button_after_interaction,
                )
                    .run_if(in_state(State::OptionsMenu)),
            )
            .add_systems(OnExit(State::OptionsMenu), cleanup_main_menu);
    }
}
