use bevy::{
    asset::Handle,
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        name::Name,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputFocus, InputFocusVisible, directional_navigation::DirectionalNavigation,
        tab_navigation::TabIndex,
    },
    math::CompassOctant,
    picking::hover::Hovered,
    prelude::{Deref, DerefMut},
    state::state_scoped::DespawnOnExit,
    text::{Font, TextColor, TextFont},
    time::{Time, Timer},
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        Interaction, JustifyContent, JustifyItems, Node, PositionType, UiRect, Val, percent, px,
        widget::{Button, Text},
    },
    ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick},
    utils::default,
};

use crate::state::State;

// ─── Color Palette ────────────────────────────────────────────────────────────

pub const BG_DARK: Color = Color::srgb(0.063, 0.063, 0.114);
pub const PANEL_BG: Color = Color::srgba(0.09, 0.09, 0.18, 0.96);
const BUTTON_NORMAL: Color = Color::srgba(0.10, 0.09, 0.18, 1.0);
const BUTTON_FOCUSED: Color = Color::srgb(0.96, 0.75, 0.13);
const BORDER_NORMAL: Color = Color::srgb(0.28, 0.22, 0.08);
const BORDER_FOCUSED: Color = Color::srgb(0.96, 0.75, 0.13);
pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.90, 0.78);
pub const TEXT_TITLE: Color = Color::srgb(0.98, 0.85, 0.50);
const SLIDER_TRACK_COLOR: Color = Color::srgb(0.14, 0.11, 0.05);
const SLIDER_THUMB_COLOR: Color = Color::srgb(0.96, 0.75, 0.13);
const TEXT_ON_FOCUSED: Color = Color::srgb(0.05, 0.04, 0.02);

// ─── Camera ───────────────────────────────────────────────────────────────────

#[derive(Component)]
pub struct MenuCamera;

pub fn spawn_camera(query: Query<Entity, With<MenuCamera>>, mut commands: Commands) {
    if query.is_empty() {
        commands.spawn(Camera2d).insert(MenuCamera);
    }
}

pub fn despawn_menu_camera(
    query: Query<Entity, With<MenuCamera>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// ─── Layout Helpers ───────────────────────────────────────────────────────────

pub fn spawn_menu_root(commands: &mut Commands, state: State, bg_color: Color) -> Entity {
    commands
        .spawn((
            DespawnOnExit(state),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .id()
}

pub fn spawn_panel(commands: &mut Commands, state: State, bg_color: Color) -> Entity {
    commands
        .spawn((
            DespawnOnExit(state),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::axes(Val::Px(32.0), Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .id()
}

pub fn spawn_title(
    commands: &mut Commands,
    text: &str,
    state: State,
    font: Handle<Font>,
) -> Entity {
    commands
        .spawn((
            DespawnOnExit(state),
            Text::new(text),
            TextFont { font, font_size: 52.0, ..default() },
            TextColor(TEXT_TITLE),
        ))
        .id()
}

pub fn spawn_divider(commands: &mut Commands, state: State) -> Entity {
    commands
        .spawn((
            DespawnOnExit(state),
            Node {
                width: Val::Px(200.0),
                height: Val::Px(2.0),
                ..default()
            },
            BackgroundColor(BORDER_FOCUSED),
        ))
        .id()
}

// ─── Button ───────────────────────────────────────────────────────────────────

pub fn get_button_bundle(name: String) -> (Button, Node, BackgroundColor, BorderColor, Name) {
    (
        Button,
        Node {
            width: Val::Px(280.),
            height: Val::Px(56.),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BUTTON_NORMAL),
        BorderColor::all(BORDER_NORMAL),
        Name::new(name),
    )
}

pub fn button_text(text: &str, font: Handle<Font>) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont { font, font_size: 20.0, ..default() },
        TextColor(TEXT_PRIMARY),
    )
}

// ─── Focus System ─────────────────────────────────────────────────────────────

pub fn highlight_focused_element(
    mut input_focus: ResMut<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut buttons: Query<(Entity, &Interaction, &mut BackgroundColor, &mut BorderColor, Option<&Children>), With<Button>>,
    mut texts: Query<&mut TextColor>,
) {
    for (entity, interaction, mut bg, mut border, children) in buttons.iter_mut() {
        let is_hovered = matches!(interaction, Interaction::Hovered | Interaction::Pressed);
        let is_focused = input_focus.0 == Some(entity) && input_focus_visible.0;

        if is_hovered {
            input_focus.0 = Some(entity);
        }

        if is_focused || is_hovered {
            *bg = BackgroundColor(BUTTON_FOCUSED);
            *border = BorderColor::all(BORDER_FOCUSED);
            if let Some(children) = children {
                for &child in children.iter() {
                    if let Ok(mut tc) = texts.get_mut(child) {
                        tc.0 = TEXT_ON_FOCUSED;
                    }
                }
            }
        } else {
            *bg = BackgroundColor(BUTTON_NORMAL);
            *border = BorderColor::all(BORDER_NORMAL);
            if let Some(children) = children {
                for &child in children.iter() {
                    if let Ok(mut tc) = texts.get_mut(child) {
                        tc.0 = TEXT_PRIMARY;
                    }
                }
            }
        }
    }
}

// ─── Navigation ───────────────────────────────────────────────────────────────

pub fn navigate(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut directional_navigation: DirectionalNavigation,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        directional_navigation.navigate(CompassOctant::North).ok();
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        directional_navigation.navigate(CompassOctant::South).ok();
    }
}

// ─── Reset Button ─────────────────────────────────────────────────────────────

#[derive(Component, Default, Deref, DerefMut)]
pub struct ResetTimer(Timer);

pub fn reset_button_after_interaction(
    time: Res<Time>,
    mut query: Query<(&mut ResetTimer, &mut BackgroundColor)>,
) {
    for (mut reset_timer, mut color) in query.iter_mut() {
        reset_timer.tick(time.delta());
        if reset_timer.just_finished() {
            color.0 = BUTTON_NORMAL;
        }
    }
}

// ─── Slider ───────────────────────────────────────────────────────────────────

pub fn spawn_slider(
    commands: &mut Commands,
    name: String,
    value: f32,
    min: f32,
    max: f32,
) -> Entity {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                justify_items: JustifyItems::Center,
                column_gap: px(4),
                height: px(12),
                width: Val::Px(280.),
                ..default()
            },
            Name::new(name),
            Hovered::default(),
            Slider {
                track_click: TrackClick::Snap,
            },
            SliderValue(value),
            SliderRange::new(min, max),
            TabIndex(0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK_COLOR),
            ));
            parent
                .spawn(Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                })
                .with_child((
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB_COLOR),
                ));
        })
        .id()
}
