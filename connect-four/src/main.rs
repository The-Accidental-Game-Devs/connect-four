mod assets;
mod states;

use bevy::prelude::*;
use bevy::window::EnabledButtons;
use assets::Assets;
use states::AppState;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Connect Four".into(),
                name: Some("Connect Four".into()),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    minimize: true,
                    maximize: false,
                    close: true,
                },
                ..default()
            }),
            ..default()
        }),))
        .insert_state(AppState::Loading)
        .add_systems(Startup, setup)
        .add_systems(Startup, load_assets)
        .add_systems(
            Update,
            check_assets_loaded.run_if(in_state(AppState::Loading)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        board: asset_server.load("board.png"),
        red_piece: asset_server.load("red_piece.png"),
        yellow_piece: asset_server.load("yellow_piece.png"),
    });
}

fn check_assets_loaded(
    assets_server: Res<AssetServer>,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if assets_server.is_loaded_with_dependencies(&assets.board)
        && assets_server.is_loaded_with_dependencies(&assets.red_piece)
        && assets_server.is_loaded_with_dependencies(&assets.yellow_piece)
    {
        next_state.set(AppState::InGame);
    }
}