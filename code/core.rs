use bevy::prelude::*;
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState { #[default] MainMenu, InGame, Scene }
