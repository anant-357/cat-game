use bevy::{
    camera::Camera2d,
    color::{Color, Srgba},
    ecs::{
        children,
        component::Component,
        entity::Entity,
        hierarchy::{ChildOf, Children},
        name::Name,
        query::With,
        spawn::{Spawn, SpawnRelated, SpawnRelatedBundle},
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{
        InputFocus, InputFocusVisible, directional_navigation::DirectionalNavigation,
        tab_navigation::TabIndex,
    },
    math::CompassOctant,
    picking::hover::Hovered,
    prelude::{Deref, DerefMut},
    time::{Time, Timer},
    ui::{
        AlignItems, AlignSelf, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        JustifyContent, JustifyItems, JustifySelf, Node, PositionType, UiRect, Val, percent, px,
        widget::Button,
    },
    ui_widgets::{CoreSliderDragState, Slider, SliderRange, SliderThumb, SliderValue, TrackClick},
    utils::default,
};

const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_400;
const FOCUSED_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_500;
const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MenuCamera;

pub fn get_button_bundle(name: String) -> (Button, Node, BackgroundColor, Name) {
    return (
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
        Name::new(name),
    );
}

pub fn get_slider_bundle(
    name: String,
    value: f32,
    min: f32,
    max: f32,
) -> (
    Node,
    Name,
    Hovered,
    Slider,
    SliderValue,
    SliderRange,
    TabIndex,
    SpawnRelatedBundle<
        ChildOf,
        (
            Spawn<(Node, BackgroundColor, BorderRadius)>,
            Spawn<(
                bevy::prelude::Node,
                SpawnRelatedBundle<
                    ChildOf,
                    Spawn<(
                        SliderThumb,
                        bevy::prelude::Node,
                        BorderRadius,
                        BackgroundColor,
                    )>,
                >,
            )>,
        ),
    >,
) {
    return (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(30),
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
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: px(6),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK), // Border color for the slider
                BorderRadius::all(px(3)),
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 12px to accommodate the thumb.
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0), // This will be updated by the slider's value
                        ..default()
                    },
                    BorderRadius::MAX,
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    );
}

pub fn navigate(
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

pub fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut BorderColor)>,
) {
    for (entity, mut border_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            border_color.set_all(Color::Srgba(FOCUSED_BUTTON));
        } else {
            border_color.set_all(Color::NONE);
        }
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct ResetTimer(Timer);

pub fn reset_button_after_interaction(
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

pub fn spawn_camera(query: Query<Entity, With<MenuCamera>>, mut commands: Commands) {
    if query.is_empty() {
        commands.spawn(Camera2d).insert(MenuCamera);
    }
}
