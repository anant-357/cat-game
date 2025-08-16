use bevy::prelude::*;
use strum::{EnumCount, EnumIter, IntoStaticStr};

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, EnumIter, IntoStaticStr, EnumCount,
)]
pub enum State {
    #[default]
    Loading,
    MainMenu,
    OptionsMenu,
    Paused,
    Playing,
    ChooseArea,
}
