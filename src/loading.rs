use bevy::{
    app::Plugin,
    ecs::system::{Commands, ResMut},
    state::{
        commands,
        state::{NextState, OnEnter, OnExit},
    },
};

use crate::state::State;

fn setup(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::MainMenu);
}

fn cleanup() {}

#[derive(Default)]
pub struct LoadingState {}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(State::Loading), setup)
            .add_systems(OnExit(State::Loading), cleanup);
    }
}
