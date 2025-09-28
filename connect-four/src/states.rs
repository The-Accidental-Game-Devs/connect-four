use bevy::prelude::States;

#[derive(States, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

#[derive(States, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub enum GameState {
    #[default]
    Setup,
    WhoTurn,
    PlayerInput,
    BotInput,
    SimulateGravity,
    IsGameOver,
    GameOver,
    Replay,
}
