use bevy::prelude::States;

#[derive(States, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub enum AppState {
    #[default]
    Loading,
    SetupGame,
    WhoTurn,
    PlayerInput,
    BotInput,
    IsGameOver,
    GameOver,
    Replay,
}
