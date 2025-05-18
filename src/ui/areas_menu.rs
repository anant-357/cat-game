use bevy::{
    app::{Plugin, PreUpdate, Update},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        name::Name,
        query::With,
        relationship::RelationshipSourceCollection,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{InputFocus, directional_navigation::DirectionalNavigationMap},
    math::CompassOctant,
    state::{
        condition::in_state,
        state::{NextState, OnEnter, OnExit},
    },
    ui::{Display, Node, RepeatedGridTrack, Val, widget::Text},
    utils::default,
};
use strum::{EnumCount, IntoEnumIterator};

use crate::{area::Area, state::State};

use super::common::{
    get_button_bundle, highlight_focused_element, navigate, reset_button_after_interaction,
};

#[derive(Component)]
pub struct Areas;

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands.spawn(Camera2d).insert(Areas);

    let root_node = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(Areas)
        .id();

    let area_count = Area::COUNT;

    let grid_root_entity = commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            grid_template_columns: RepeatedGridTrack::auto(1),
            grid_template_rows: RepeatedGridTrack::auto((area_count + 1) as u16),
            ..default()
        })
        .id();

    commands.entity(root_node).add_child(grid_root_entity);
    let mut button_entities: Vec<Entity> = Vec::new();

    for area in Area::iter() {
        let name: &'static str = area.into();
        let button = commands
            .spawn(get_button_bundle(name.to_string()))
            .with_child(Text::new(name.to_string()))
            .id();
        commands.entity(grid_root_entity).add_child(button);
        button_entities.add(button);
    }

    let exit_button_entity = commands
        .spawn(get_button_bundle("Main Menu".to_string()))
        .with_child(Text::new("Main Menu"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(exit_button_entity);

    button_entities.add(exit_button_entity);
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
                    match name.as_str() {
                        "Option 1" => {
                            next_state.set(State::Playing);
                        }
                        "Option 2" => {
                            next_state.set(State::OptionsMenu);
                        }
                        "Main Menu" => {
                            next_state.set(State::MainMenu);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<Areas>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub struct AreasMenuPlugin;
impl Plugin for AreasMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(State::ChooseArea), setup_ui)
            .add_systems(PreUpdate, navigate.run_if(in_state(State::ChooseArea)))
            .add_systems(
                Update,
                (
                    highlight_focused_element,
                    interact_with_focused_button,
                    reset_button_after_interaction,
                )
                    .run_if(in_state(State::ChooseArea)),
            )
            .add_systems(OnExit(State::ChooseArea), cleanup_main_menu);
    }
}
