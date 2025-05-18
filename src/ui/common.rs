use bevy::{
    color::{Color, Srgba},
    ecs::{
        component::Component,
        entity::Entity,
        name::Name,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    input_focus::{InputFocus, InputFocusVisible, directional_navigation::DirectionalNavigation},
    math::CompassOctant,
    prelude::{Deref, DerefMut},
    time::{Time, Timer},
    ui::{
        AlignItems, AlignSelf, BackgroundColor, BorderColor, JustifyContent, JustifySelf, Node,
        UiRect, Val, widget::Button,
    },
    utils::default,
};

const NORMAL_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_400;
const FOCUSED_BUTTON: Srgba = bevy::color::palettes::tailwind::AMBER_500;

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
            border_color.0 = FOCUSED_BUTTON.into();
        } else {
            border_color.0 = Color::NONE;
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
