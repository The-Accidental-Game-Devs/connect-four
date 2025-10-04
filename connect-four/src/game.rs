use crate::assets::Assets;
use crate::game_difficulty::{GameDifficulty, GameDifficultyResource};
use crate::game_result::{GameResult, GameResultResource};
use crate::settings::*;
use crate::states::{AppState, GameState};
use bevy::prelude::*;
use connect_four_engine::bitboard::*;
use connect_four_engine::bot::find_best_move;

#[derive(Resource)]
struct GameData {
    game_board: Bitboard,
    player_board: Bitboard,
    bot_board: Bitboard,
    player_col: usize,
    bot_col: usize,
    player_turn: bool,
}

#[derive(Component)]
struct ActivePiece {
    col: usize,
}

#[derive(Component)]
struct Piece {}

#[derive(Component)]
struct Falling {
    end_position: Vec3,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(
            OnEnter(GameState::WhoTurn),
            check_who_turn.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            handle_player_move_input
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::PlayerInput)),
        );
        app.add_systems(
            OnEnter(GameState::PlayerInput),
            unhide_active_piece.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            handle_player_drop_input
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::PlayerInput)),
        );
        app.add_systems(
            OnExit(GameState::PlayerInput),
            hide_active_piece.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnEnter(GameState::BotInput),
            handle_bot_input.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnEnter(GameState::DropPiece),
            drop_piece.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            Update,
            simulate_gravity
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::SimulateGravity)),
        );
        app.add_systems(
            Update,
            is_reached
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::SimulateGravity)),
        );
        app.add_systems(
            OnEnter(GameState::IsGameOver),
            check_is_game_over.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnEnter(GameState::NextTurn),
            next_turn.run_if(in_state(AppState::InGame)),
        );
        app.add_systems(
            OnEnter(GameState::Replay),
            handle_replay.run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(GameData {
        game_board: 0,
        player_board: 0,
        bot_board: 0,
        player_col: 3,
        bot_col: 3,
        player_turn: true,
    });
    commands.insert_resource(GameResultResource {
        game_result: GameResult::Unknow,
    });
    commands.spawn((
        Sprite::from_image(assets.board.clone()),
        DespawnOnExit(AppState::InGame),
    ));
    commands.spawn((
        Sprite::from_image(assets.board_border.clone()),
        DespawnOnExit(AppState::InGame),
    ));
    commands.spawn((
        ActivePiece { col: 3 },
        Sprite::from_image(assets.yellow_piece.clone()),
        Transform::from_xyz(0.0, HALF_BOARD_HEIGHT + HALF_PIECE_SIZE, 0.0),
        Visibility::Visible,
        DespawnOnExit(AppState::InGame),
    ));
    next_state.set(GameState::WhoTurn);
}

fn check_who_turn(game_data: Res<GameData>, mut next_state: ResMut<NextState<GameState>>) {
    if game_data.player_turn {
        next_state.set(GameState::PlayerInput);
    } else {
        next_state.set(GameState::BotInput);
    }
}

fn handle_player_move_input(
    mut query: Query<(&mut ActivePiece, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut active_piece, mut transform)) = query.single_mut() {
        if keys.just_pressed(KeyCode::KeyD) {
            if active_piece.col < COLS - 1 {
                active_piece.col += 1;
                transform.translation.x =
                    active_piece.col as f32 * PIECE_SIZE - HALF_BOARD_WIDTH + HALF_PIECE_SIZE;
            }
        }
        if keys.just_pressed(KeyCode::KeyA) {
            if active_piece.col > 0 {
                active_piece.col -= 1;
                transform.translation.x =
                    active_piece.col as f32 * PIECE_SIZE - HALF_BOARD_WIDTH + HALF_PIECE_SIZE;
            }
        }
    }
}

fn unhide_active_piece(mut query: Query<&mut Visibility, With<ActivePiece>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Visible;
    }
}

fn handle_player_drop_input(
    query: Query<&ActivePiece>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(active_piece) = query.single() {
        if keys.just_pressed(KeyCode::Space) {
            if can_place(game_data.game_board, active_piece.col) {
                game_data.player_col = active_piece.col;
                next_state.set(GameState::DropPiece);
            }
        }
    }
}

fn hide_active_piece(mut query: Query<&mut Visibility, With<ActivePiece>>) {
    if let Ok(mut visibility) = query.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn handle_bot_input(
    mut game_data: ResMut<GameData>,
    mut game_result_resource: ResMut<GameResultResource>,
    game_difficulty_resource: Res<GameDifficultyResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let depth = match game_difficulty_resource.game_difficulty {
        GameDifficulty::Easy => 4,
        GameDifficulty::Normal => 8,
        GameDifficulty::Hard => 12,
    };
    let Some(best_move) = find_best_move(
        game_data.game_board,
        game_data.player_board,
        game_data.bot_board,
        depth,
    ) else {
        game_result_resource.game_result = GameResult::Unknow;
        next_state.set(GameState::GameOver);
        return;
    };
    game_data.bot_col = best_move;
    next_state.set(GameState::DropPiece);
}

fn drop_piece(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut game_result_resource: ResMut<GameResultResource>,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let next_row: Bitboard;
    if game_data.player_turn {
        next_row = get_next_row(game_data.game_board, game_data.player_col);
        game_data.player_board ^= next_row;
    } else {
        next_row = get_next_row(game_data.game_board, game_data.bot_col);
        game_data.bot_board ^= next_row;
    }
    game_data.game_board |= next_row;

    let Some((row, col)) = indices_from_bitmask(next_row) else {
        game_result_resource.game_result = GameResult::Unknow;
        next_state.set(GameState::GameOver);
        return;
    };

    let start_x = col as f32 * PIECE_SIZE - HALF_BOARD_WIDTH + HALF_PIECE_SIZE;
    let start_y = HALF_BOARD_HEIGHT + HALF_PIECE_SIZE;
    let end_x = start_x;
    let end_y = row as f32 * PIECE_SIZE - HALF_BOARD_HEIGHT + HALF_PIECE_SIZE;

    commands.spawn((
        Piece {},
        Falling {
            end_position: Vec3 {
                x: end_x,
                y: end_y,
                z: -1.0,
            },
        },
        Sprite::from_image(if game_data.player_turn {
            assets.yellow_piece.clone()
        } else {
            assets.red_piece.clone()
        }),
        Transform::from_xyz(start_x, start_y, -1.0),
        DespawnOnExit(AppState::InGame),
    ));

    next_state.set(GameState::SimulateGravity);
}

fn simulate_gravity(time: Res<Time>, mut query: Query<(&Falling, &mut Transform)>) {
    if let Ok((falling, mut transform)) = query.single_mut() {
        let delta_time = time.delta_secs().min(0.003);
        let direction = falling.end_position - transform.translation;
        let distance = direction.length();
        let step = GRAVITY * delta_time;
        if distance > step {
            transform.translation += direction.normalize() * step;
        } else {
            transform.translation = falling.end_position;
        }
    }
}

fn is_reached(
    mut commands: Commands,
    query: Query<(Entity, &Falling, &Transform)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, falling, transform)) = query.single() {
        if transform.translation == falling.end_position {
            commands.entity(entity).remove::<Falling>();
            next_state.set(GameState::IsGameOver);
        }
    }
}

fn check_is_game_over(
    game_data: Res<GameData>,
    mut game_result_resource: ResMut<GameResultResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if has_won(game_data.player_board) {
        game_result_resource.game_result = GameResult::PlayerWon;
        next_state.set(GameState::GameOver);
        return;
    }

    if has_won(game_data.bot_board) {
        game_result_resource.game_result = GameResult::BotWon;
        next_state.set(GameState::GameOver);
        return;
    }

    if is_board_full(game_data.game_board) {
        game_result_resource.game_result = GameResult::Draw;
        next_state.set(GameState::GameOver);
        return;
    }

    next_state.set(GameState::NextTurn);
}

fn next_turn(mut game_data: ResMut<GameData>, mut next_state: ResMut<NextState<GameState>>) {
    if game_data.player_turn {
        game_data.player_turn = false;
    } else {
        game_data.player_turn = true;
    }
    next_state.set(GameState::WhoTurn);
}

fn handle_replay(
    mut commands: Commands,
    query: Query<Entity, With<Piece>>,
    game_data: Res<GameData>,
    game_result_resource: Res<GameResultResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for piece in query {
        commands.entity(piece).despawn();
    }

    let player_turn = match game_result_resource.game_result {
        GameResult::PlayerWon => false,
        GameResult::BotWon => true,
        GameResult::Draw => game_data.player_turn,
        GameResult::Unknow => true,
    };

    commands.insert_resource(GameData {
        game_board: 0,
        player_board: 0,
        bot_board: 0,
        player_col: 3,
        bot_col: 3,
        player_turn: player_turn,
    });
    commands.insert_resource(GameResultResource {
        game_result: GameResult::Unknow,
    });

    next_state.set(GameState::WhoTurn);
}
