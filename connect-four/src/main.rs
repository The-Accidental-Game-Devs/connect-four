mod assets;
mod game;
mod game_result;
mod game_ui;
mod main_menu;
mod settings;
mod states;
mod ui_settings;

use crate::states::{AppState, GameState};
use assets::Assets;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use game::GamePlugin;
use game_ui::GameUiPlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Connect Four".into(),
                    name: Some("Connect Four".into()),
                    resolution: (1280.0, 720.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            MainMenuPlugin,
            GamePlugin,
            GameUiPlugin,
        ))
        .insert_state(AppState::Loading)
        .enable_state_scoped_entities::<AppState>()
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
                min_width: 1280.0,
                min_height: 720.0,
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
