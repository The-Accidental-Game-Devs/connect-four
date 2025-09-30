use bevy::prelude::Resource;

#[derive(PartialEq)]
pub enum Result {
    PlayerWon,
    BotWon,
    Draw,
    Unknow,
}

#[derive(Resource)]
pub struct GameResult {
    pub result: Result,
}
