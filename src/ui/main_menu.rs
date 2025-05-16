use bevy::{
    app::{Plugin, PreUpdate, Update},
    color::{Color, Srgba},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        name::Name,
        query::{self, With},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputFocus, InputFocusVisible,
        directional_navigation::{self, DirectionalNavigation, DirectionalNavigationMap},
    },
    math::{CompassOctant, CompassQuadrant},
    reflect::List,
    state::{
        commands,
        condition::in_state,
        state::{NextState, OnEnter, OnExit},
    },
    text::TextLayout,
    ui::{
        AlignItems, AlignSelf, BackgroundColor, BorderColor, BorderRadius, Display, JustifyContent,
        JustifySelf, Node, RepeatedGridTrack, UiRect, Val,
        widget::{Button, Text},
    },
    utils::default,
};

use crate::state::State;

const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_400;
const FOCUSED_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_500;
const PRESSED_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_50;

#[derive(Component)]
pub struct MainMenu;

fn setup_main_menu(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
) {
    commands.spawn(Camera2d).insert(MainMenu);
    let root_node = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        })
        .insert(MainMenu)
        .id();
    let grid_root_entry = commands
        .spawn(Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::auto(1),
            grid_template_rows: RepeatedGridTrack::auto(2),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        })
        .id();

    commands.entity(root_node).add_child(grid_root_entry);

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
            Name::new("Play"),
        ))
        .with_child(Text::new("Play"))
        .id();
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
            Name::new("Options"),
        ))
        .with_child(Text::new("Options"))
        .id();
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
            Name::new("Exit"),
        ))
        .with_child(Text::new("Exit"))
        .id();

    let button_entities: Vec<Entity> = Vec::from([
        play_button_entity,
        options_button_entity,
        exit_button_entity,
    ]);
    commands
        .entity(grid_root_entry)
        .add_children(&button_entities);
    directional_nav_map.add_edges(&button_entities, CompassOctant::South);
    let top = button_entities[0];
    input_focus.set(top);
}

fn highlight_focused_button(
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

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn navigate(
    mut directional_navigation: DirectionalNavigation,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        directional_navigation
            .navigate(CompassOctant::North)
            .ok()
            .unwrap();
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        directional_navigation
            .navigate(CompassOctant::South)
            .ok()
            .unwrap();
    }
}

fn interact(mut next_state: ResMut<NextState<State>>, keyboard_input: Res<ButtonInput<KeyCode>>) {}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(InputFocusVisible(true))
            .add_systems(OnEnter(State::MainMenu), setup_main_menu)
            .add_systems(PreUpdate, navigate)
            .add_systems(
                Update,
                (highlight_focused_button, interact).run_if(in_state(State::MainMenu)),
            )
            .add_systems(OnExit(State::MainMenu), cleanup_main_menu);
    }
}
