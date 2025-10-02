use bevy::prelude::Resource;

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Resource)]
pub struct GameDifficulty {
    pub difficulty: Difficulty,
}
