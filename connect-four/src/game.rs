use crate::assets::Assets;
use crate::game_difficulty::*;
use crate::game_result::{GameResult, Result};
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

#[derive(Component)]
struct PhysicsPosition(Vec3);

#[derive(Component)]
struct PrevPhysicsPosition(Vec3);

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
            FixedUpdate,
            simulate_gravity
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::SimulateGravity)),
        );
        app.add_systems(
            Update,
            interpolate_visual
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
        player_turn: true,
    });
    commands.insert_resource(GameResult {
        result: Result::Unknow,
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

fn check_who_turn(mut game_data: ResMut<GameData>, mut next_state: ResMut<NextState<GameState>>) {
    if game_data.player_turn {
        next_state.set(GameState::PlayerInput);
        game_data.player_turn = !game_data.player_turn;
    } else {
        next_state.set(GameState::BotInput);
        game_data.player_turn = !game_data.player_turn;
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
    mut commands: Commands,
    query: Query<(&ActivePiece, &Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut game_result: ResMut<GameResult>,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((active_piece, transform)) = query.single() {
        if keys.just_pressed(KeyCode::Space) {
            if can_place(game_data.game_board, active_piece.col) {
                let next_row = get_next_row(game_data.game_board, active_piece.col);
                game_data.game_board |= next_row;
                game_data.player_board ^= next_row;

                let Some((row, col)) = indices_from_bitmask(next_row) else {
                    game_result.result = Result::Unknow;
                    next_state.set(GameState::GameOver);
                    return;
                };

                let x = col as f32 * PIECE_SIZE - HALF_BOARD_WIDTH + HALF_PIECE_SIZE;
                let y = row as f32 * PIECE_SIZE - HALF_BOARD_HEIGHT + HALF_PIECE_SIZE;

                commands.spawn((
                    Piece {},
                    Falling {
                        end_position: Vec3 {
                            x: x,
                            y: y,
                            z: -1.0,
                        },
                    },
                    Sprite::from_image(assets.yellow_piece.clone()),
                    Transform::from_xyz(x, transform.translation.y, -1.0),
                    PhysicsPosition(Vec3 {
                        x: x,
                        y: transform.translation.y,
                        z: -1.0,
                    }),
                    PrevPhysicsPosition(Vec3 {
                        x: x,
                        y: transform.translation.y,
                        z: -1.0,
                    }),
                    DespawnOnExit(AppState::InGame),
                ));

                next_state.set(GameState::SimulateGravity);
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
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut game_result: ResMut<GameResult>,
    game_difficulty: Res<GameDifficulty>,
    assets: Res<Assets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let depth = match game_difficulty.difficulty {
        Difficulty::Easy => 4,
        Difficulty::Normal => 8,
        Difficulty::Hard => 12,
    };
    let Some(best_move) = find_best_move(
        game_data.game_board,
        game_data.player_board,
        game_data.bot_board,
        depth,
    ) else {
        game_result.result = Result::Unknow;
        next_state.set(GameState::GameOver);
        return;
    };

    let next_row = get_next_row(game_data.game_board, best_move);
    game_data.game_board |= next_row;
    game_data.bot_board ^= next_row;

    let Some((row, col)) = indices_from_bitmask(next_row) else {
        game_result.result = Result::Unknow;
        next_state.set(GameState::GameOver);
        return;
    };

    let x = col as f32 * PIECE_SIZE - HALF_BOARD_WIDTH + HALF_PIECE_SIZE;
    let y = row as f32 * PIECE_SIZE - HALF_BOARD_HEIGHT + HALF_PIECE_SIZE;

    commands.spawn((
        Piece {},
        Falling {
            end_position: Vec3 {
                x: x,
                y: y,
                z: -1.0,
            },
        },
        Sprite::from_image(assets.red_piece.clone()),
        Transform::from_xyz(x, HALF_BOARD_HEIGHT + HALF_PIECE_SIZE, -1.0),
        PhysicsPosition(Vec3 {
            x: x,
            y: HALF_BOARD_HEIGHT + HALF_PIECE_SIZE,
            z: -1.0,
        }),
        PrevPhysicsPosition(Vec3 {
            x: x,
            y: HALF_BOARD_HEIGHT + HALF_PIECE_SIZE,
            z: -1.0,
        }),
        DespawnOnExit(AppState::InGame),
    ));

    next_state.set(GameState::SimulateGravity);
}

fn simulate_gravity(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&Falling, &mut PhysicsPosition, &mut PrevPhysicsPosition)>,
) {
    let delta_time = time_fixed.delta_secs();
    if let Ok((falling, mut current_pos, mut prev_pos)) = query.single_mut() {
        prev_pos.0 = current_pos.0;

        let direction = falling.end_position - current_pos.0;
        let distance = direction.length();
        let step = GRAVITY * delta_time;

        if distance > step {
            current_pos.0 += direction.normalize() * step;
        } else {
            current_pos.0 = falling.end_position;
        }
    }
}

fn interpolate_visual(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(
        &Falling,
        &PhysicsPosition,
        &PrevPhysicsPosition,
        &mut Transform,
    )>,
) {
    let alpha = time_fixed.overstep_fraction();

    if let Ok((_falling, current_pos, prev_pos, mut transform)) = query.single_mut() {
        transform.translation = prev_pos.0.lerp(current_pos.0, alpha);
    }
}

fn is_reached(
    mut commands: Commands,
    query: Query<(Entity, &Falling, &Transform)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, falling, transform)) = query.single() {
        if transform.translation == falling.end_position {
            commands
                .entity(entity)
                .remove::<(PhysicsPosition, PrevPhysicsPosition, Falling)>();
            next_state.set(GameState::IsGameOver);
        }
    }
}

fn check_is_game_over(
    game_data: Res<GameData>,
    mut game_result: ResMut<GameResult>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if has_won(game_data.player_board) {
        game_result.result = Result::PlayerWon;
        next_state.set(GameState::GameOver);
        return;
    }

    if has_won(game_data.bot_board) {
        game_result.result = Result::BotWon;
        next_state.set(GameState::GameOver);
        return;
    }

    if is_board_full(game_data.game_board) {
        game_result.result = Result::Draw;
        next_state.set(GameState::GameOver);
        return;
    }

    next_state.set(GameState::WhoTurn);
}

fn handle_replay(
    mut commands: Commands,
    query: Query<Entity, With<Piece>>,
    game_data: Res<GameData>,
    game_result: Res<GameResult>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for piece in query {
        commands.entity(piece).despawn();
    }

    let player_turn = match game_result.result {
        Result::PlayerWon => false,
        Result::BotWon => true,
        Result::Draw => game_data.player_turn,
        Result::Unknow => true,
    };

    commands.insert_resource(GameData {
        game_board: 0,
        player_board: 0,
        bot_board: 0,
        player_turn: player_turn,
    });
    commands.insert_resource(GameResult {
        result: Result::Unknow,
    });

    next_state.set(GameState::WhoTurn);
}
