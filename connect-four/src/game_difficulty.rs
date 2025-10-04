use bevy::prelude::Resource;

#[derive(PartialEq)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Resource)]
pub struct GameDifficultyResource {
    pub game_difficulty: GameDifficulty,
}
