use bevy::prelude::*;

use crate::state::State;

// TODO: re-enable when cat.glb is available
// #[derive(Resource)]
// pub struct CatAssets {
//     pub gltf: Handle<bevy::gltf::Gltf>,
// }

fn start_loading(mut next_state: ResMut<NextState<State>>) {
    // TODO: load cat.glb here and wait for it before transitioning
    next_state.set(State::MainMenu);
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::Loading), start_loading);
    }
}
