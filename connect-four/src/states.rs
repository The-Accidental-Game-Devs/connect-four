use bevy::prelude::States;

#[derive(States, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    SetupGame,
    WhoTurn,
    PlayerInput,
    BotInput,
    SimulateGravity,
    IsGameOver,
    GameOver,
    Replay,
}
