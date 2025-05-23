use bevy::{
    app::{AppExit, Plugin, PreUpdate, Update},
    color::Srgba,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
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
        state::{NextState, OnEnter, OnExit},
    },
    ui::{Display, Node, RepeatedGridTrack, Val, widget::Text},
    utils::default,
};

use crate::state::State;

use super::common::{
    get_button_bundle, highlight_focused_element, navigate, reset_button_after_interaction,
};

#[derive(Component)]
pub struct MainMenu;

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands.spawn(Camera2d).insert(MainMenu);

    let root_node = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(MainMenu)
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

    let play_button_entity = commands
        .spawn(get_button_bundle("Play".into()))
        .with_child(Text::new("Play"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(play_button_entity);

    let areas_button_entity = commands
        .spawn(get_button_bundle("Choose Area".into()))
        .with_child(Text::new("Choose Area"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(areas_button_entity);

    let options_button_entity = commands
        .spawn(get_button_bundle("Options".into()))
        .with_child(Text::new("Options"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(options_button_entity);

    let exit_button_entity = commands
        .spawn(get_button_bundle("Exit".into()))
        .with_child(Text::new("Exit"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(exit_button_entity);

    let button_entities: Vec<Entity> = Vec::from([
        play_button_entity,
        areas_button_entity,
        options_button_entity,
        exit_button_entity,
    ]);
    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    let top = button_entities[0];
    input_focus.set(top);
}

fn interact_with_focused_button(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
    query: Query<(Entity, &Name), With<Name>>,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<State>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(focused_entity) = input_focus.0 {
            for (e, name) in query.iter() {
                if focused_entity == e {
                    match name.as_str() {
                        "Play" => {
                            next_state.set(State::Playing);
                        }
                        "Choose Area" => {
                            next_state.set(State::ChooseArea);
                        }
                        "Options" => {
                            next_state.set(State::OptionsMenu);
                        }
                        "Exit" => {
                            exit.write(AppExit::Success);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin))
            .insert_resource(InputFocusVisible(true))
            .add_systems(OnEnter(State::MainMenu), setup_ui)
            .add_systems(PreUpdate, navigate.run_if(in_state(State::MainMenu)))
            .add_systems(
                Update,
                (
                    highlight_focused_element,
                    interact_with_focused_button,
                    reset_button_after_interaction,
                )
                    .run_if(in_state(State::MainMenu)),
            )
            .add_systems(OnExit(State::MainMenu), cleanup_main_menu);
    }
}
