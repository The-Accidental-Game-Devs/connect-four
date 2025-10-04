use bevy::prelude::Resource;

#[derive(PartialEq)]
pub enum GameResult {
    PlayerWon,
    BotWon,
    Draw,
    Unknow,
}

#[derive(Resource)]
pub struct GameResultResource {
    pub game_result: GameResult,
}
