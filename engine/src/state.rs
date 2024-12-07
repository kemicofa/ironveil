use bevy::prelude::States;

#[derive(Default, Debug, States, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
}
