use crate::bitboard::*;
use rayon::prelude::*;

pub const MIDDLE_MASK: Bitboard = 0b111111000000000000000000000;
pub const MIDDLE_SCORE: f32 = 0.25;
pub const WIN_SCORE: f32 = 10000.0;
pub const CONNECT_SCORE: [f32; 2] = [
    1.0, // Connect two sore
    8.0, // Connect three score
];

fn evaluate(bitboard: Bitboard) -> f32 {
    if has_won(bitboard) {
        return WIN_SCORE;
    }
    let mut total_score: f32 = 0.0;

    let middle = bitboard & MIDDLE_MASK;
    total_score += middle.count_ones() as f32 * MIDDLE_SCORE;

    let mut horizontal = bitboard & (bitboard << (ROWS + 1));
    let mut vertical = bitboard & (bitboard << 1);
    let mut main_diagonal = bitboard & (bitboard << (ROWS + 2));
    let mut anti_diagonal = bitboard & (bitboard << ROWS);

    for i in 0..2 {
        let all_direction = horizontal | vertical | main_diagonal | anti_diagonal;
        total_score += all_direction.count_ones() as f32 * CONNECT_SCORE[i];

        horizontal &= horizontal << (ROWS + 1);
        vertical &= vertical << 1;
        main_diagonal &= main_diagonal << (ROWS + 2);
        anti_diagonal &= anti_diagonal << ROWS;
    }
    total_score
}

fn minmax(
    game_board: Bitboard,
    player_board: Bitboard,
    bot_board: Bitboard,
    depth: usize,
    mut alpha: f32,
    mut beta: f32,
    maximizing: bool,
) -> f32 {
    if depth == 0 || is_game_over(game_board, player_board, bot_board) {
        return evaluate(bot_board) - evaluate(player_board);
    }

    if maximizing {
        let mut max_eval = f32::NEG_INFINITY;
        for col in 0..COLS {
            if can_place(game_board, col) {
                let next_row = get_next_row(game_board, col);
                let new_game_board = game_board | next_row;
                let new_bot_board = bot_board ^ next_row;

                let eval_score = minmax(
                    new_game_board,
                    player_board,
                    new_bot_board,
                    depth - 1,
                    alpha,
                    beta,
                    false,
                );

                max_eval = max_eval.max(eval_score);
                alpha = alpha.max(eval_score);
                if beta <= alpha {
                    break;
                }
            }
        }
        return max_eval;
    } else {
        let mut min_eval = f32::INFINITY;
        for col in 0..COLS {
            if can_place(game_board, col) {
                let next_row = get_next_row(game_board, col);
                let new_game_board = game_board | next_row;
                let new_player_board = player_board ^ next_row;

                let eval_score = minmax(
                    new_game_board,
                    new_player_board,
                    bot_board,
                    depth - 1,
                    alpha,
                    beta,
                    true,
                );

                min_eval = min_eval.min(eval_score);
                beta = beta.min(eval_score);
                if beta <= alpha {
                    break;
                }
            }
        }
        return min_eval;
    }
}

pub fn find_best_move(
    game_board: Bitboard,
    player_board: Bitboard,
    bot_board: Bitboard,
    depth: usize,
) -> Option<usize> {
    let best_move = (0..COLS)
        .into_par_iter()
        .filter_map(|col| {
            if can_place(game_board, col) {
                let next_row = get_next_row(game_board, col);
                let new_game_board = game_board | next_row;
                let new_bot_board = bot_board ^ next_row;

                let eval_score = minmax(
                    new_game_board,
                    player_board,
                    new_bot_board,
                    depth - 1,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    false,
                );

                Some((col, eval_score))
            } else {
                None
            }
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    if let Some((col, _)) = best_move {
        Some(col)
    } else {
        None
    }
}
