use bevy::{
    app::{AppExit, Plugin, PostStartup, PreUpdate, Startup, Update},
    color::{Color, Srgba},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        name::Name,
        observer::Trigger,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputDispatchPlugin, InputFocus, InputFocusVisible,
        directional_navigation::{
            DirectionalNavigation, DirectionalNavigationMap, DirectionalNavigationPlugin,
        },
    },
    math::CompassOctant,
    prelude::{Deref, DerefMut},
    state::{
        condition::in_state,
        state::{NextState, OnExit},
    },
    time::{Time, Timer, TimerMode},
    ui::{
        AlignItems, AlignSelf, BackgroundColor, BorderColor, Display, JustifyContent, JustifySelf,
        Node, RepeatedGridTrack, UiRect, Val,
        widget::{Button, Text},
    },
    utils::default,
};

use crate::state::State;

const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_400;
const FOCUSED_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_500;

#[derive(Component)]
pub struct MainMenu;

fn setup_ui(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands.spawn(Camera2d);

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
        .spawn((
            Button,
            Node {
                width: Val::Px(200.),
                height: Val::Px(80.),
                border: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor::from(NORMAL_BUTTON),
            BorderColor::DEFAULT,
            Name::new("Play"),
        ))
        .with_child(Text::new("Play"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(play_button_entity);

    let options_button_entity = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(200.),
                height: Val::Px(80.),
                border: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor::from(NORMAL_BUTTON),
            BorderColor::DEFAULT,
            Name::new("Options"),
        ))
        .with_child(Text::new("Options"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(options_button_entity);

    let exit_button_entity = commands
        .spawn((
            Button,
            Node {
                width: Val::Px(160.),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor::from(NORMAL_BUTTON),
            BorderColor::DEFAULT,
            Name::new("Exit"),
        ))
        .with_child(Text::new("Exit"))
        .id();
    commands
        .entity(grid_root_entity)
        .add_child(exit_button_entity);

    let button_entities: Vec<Entity> = Vec::from([
        play_button_entity,
        options_button_entity,
        exit_button_entity,
    ]);
    directional_nav_map.add_looping_edges(&button_entities, CompassOctant::South);
    let top = button_entities[0];
    input_focus.set(top);
}

fn navigate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut directional_navigation: DirectionalNavigation,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        directional_navigation
            .navigate(CompassOctant::North)
            .unwrap();
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        directional_navigation
            .navigate(CompassOctant::South)
            .unwrap();
    }
}

fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut BorderColor)>,
) {
    for (entity, mut border_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            border_color.0 = FOCUSED_BUTTON.into();
        } else {
            border_color.0 = Color::NONE;
        }
    }
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

#[derive(Component, Default, Deref, DerefMut)]
struct ResetTimer(Timer);

fn reset_button_after_interaction(
    time: Res<Time>,
    mut query: Query<(&mut ResetTimer, &mut BackgroundColor)>,
) {
    for (mut reset_timer, mut color) in query.iter_mut() {
        reset_timer.tick(time.delta());
        if reset_timer.just_finished() {
            color.0 = NORMAL_BUTTON.into();
        }
    }
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((InputDispatchPlugin, DirectionalNavigationPlugin))
            .insert_resource(InputFocusVisible(true))
            .add_systems(PostStartup, setup_ui)
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
