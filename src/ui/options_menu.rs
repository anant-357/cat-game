use std::str::FromStr;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    camera::Camera2d,
    color::{Color, Gray, Srgba},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        name::Name,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputFocus, directional_navigation::DirectionalNavigationMap, tab_navigation::TabIndex,
    },
    math::CompassOctant,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, States},
        state_scoped::DespawnOnExit,
    },
    ui::{
        AlignItems, BackgroundColor, BorderRadius, Display, FlexDirection, JustifyContent,
        JustifyItems, Node, PositionType, RepeatedGridTrack, Val, percent, px, widget::Text,
    },
    ui_widgets::{CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, TrackClick},
    utils::default,
};
use strum::{EnumCount, EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use crate::{
    state::State,
    ui::common::{get_slider_bundle, spawn_camera},
};

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
    Option1,
    Option2,
    Option3,
    MainMenu,
}

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    let root_node = commands
        .spawn((
            DespawnOnExit(State::OptionsMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
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
        if name == "Option3" {
            let slider = commands
                .spawn(get_slider_bundle(name.into(), 10., 0., 100.))
                .with_child(Text::new(name.to_string()))
                .id();
            commands.entity(grid_root_entity).add_child(slider);
            button_entities.push(slider);
        } else {
            let button = commands
                .spawn(get_button_bundle(name.into()))
                .with_child(Text::new(name.to_string()))
                .id();
            commands.entity(grid_root_entity).add_child(button);
            button_entities.push(button);
        }
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
                        Ok(OptionsEnum::MainMenu) => {
                            next_state.set(State::MainMenu);
                        }
                        _ => (),
                    }
                }
            }
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
            )
                .run_if(in_state(State::OptionsMenu)),
        );
    }
}
