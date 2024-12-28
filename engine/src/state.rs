use bevy::prelude::States;

#[derive(Default, Debug, States, Hash, Eq, PartialEq, Clone)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Default, Debug, States, Hash, Eq, PartialEq, Clone)]
pub enum GameState {
    #[default]
    Paused,
    Ongoing,
    GameOver
}
