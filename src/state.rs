use crate::area::Area;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum State {
    #[default]
    MainMenu,
    OptionsMenu,
    Paused,
    Playing,
    ChooseArea,
}
