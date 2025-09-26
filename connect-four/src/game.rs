use crate::assets::Assets;
use crate::states::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);

    }
}

fn setup(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn(Sprite::from_image(assets.board.clone()));
}