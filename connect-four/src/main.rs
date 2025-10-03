mod assets;
mod game;
mod game_difficulty;
mod game_result;
mod game_ui;
mod main_menu;
mod settings;
mod states;
mod ui_settings;

use crate::states::{AppState, GameState};
use assets::Assets;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PresentMode;
use game::GamePlugin;
use game_difficulty::*;
use game_ui::GameUiPlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Connect Four".into(),
                    name: Some("Connect Four".into()),
                    resolution: (1280, 720).into(),
                    resizable: true,
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            MainMenuPlugin,
            GamePlugin,
            GameUiPlugin,
        ))
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(GameDifficulty {
            difficulty: Difficulty::Easy,
        })
        .insert_state(AppState::Loading)
        .insert_state(GameState::Setup)
        .add_systems(Startup, setup)
        .add_systems(Startup, load_assets)
        .add_systems(
            Update,
            check_assets_loaded.run_if(in_state(AppState::Loading)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1080.0,
                min_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        board: asset_server.load("board.png"),
        board_border: asset_server.load("board_border.png"),
        red_piece: asset_server.load("red_piece.png"),
        yellow_piece: asset_server.load("yellow_piece.png"),
        font: asset_server.load("Roboto/static/Roboto-Regular.ttf"),
        bold_font: asset_server.load("Roboto/static/Roboto-Bold.ttf"),
    });
}

fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if asset_server.is_loaded_with_dependencies(&assets.board)
        && asset_server.is_loaded_with_dependencies(&assets.board_border)
        && asset_server.is_loaded_with_dependencies(&assets.red_piece)
        && asset_server.is_loaded_with_dependencies(&assets.yellow_piece)
        && asset_server.is_loaded_with_dependencies(&assets.font)
        && asset_server.is_loaded_with_dependencies(&assets.bold_font)
    {
        next_state.set(AppState::MainMenu);
    }
}
