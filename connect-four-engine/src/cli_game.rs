use crate::bitboard::*;
use crate::bot::*;
use std::io;
use std::process;

const PLAYER1_SYMBOL: &str = "\x1b[32mO\x1b[0m";
const PLAYER2_SYMBOL: &str = "\x1b[31mO\x1b[0m";

pub fn print_board(player1_board: Bitboard, player2_board: Bitboard) {
    for row in (0..6).rev() {
        for col in 0..COLS {
            let shift = row + (col * COLS);
            let player1_bit = (player1_board >> shift) & 1;
            let player2_bit = (player2_board >> shift) & 1;

            if player1_bit == 1 {
                print!("{} ", PLAYER1_SYMBOL);
            } else if player2_bit == 1 {
                print!("{} ", PLAYER2_SYMBOL);
            } else {
                print!("O ");
            }
        }
        println!();
    }

    println!();

    for col in 0..COLS {
        print!("{} ", col);
    }
    println!();
}

pub fn player_vs_bot(depth: usize) {
    let mut game_board: Bitboard = 0;
    let mut player_board: Bitboard = 0;
    let mut bot_board: Bitboard = 0;
    let mut player_turn: bool = true;
    let mut game_over: bool = false;

    loop {
        if !game_over {
            if player_turn {
                print_board(player_board, bot_board);
                println!("Enter your move: ");
                let mut player_input: String = String::new();
                io::stdin()
                    .read_line(&mut player_input)
                    .expect("Failed to read line");

                let player_input: usize = match player_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a number.");
                        continue;
                    }
                };

                if can_place(game_board, player_input) {
                    let next_row = get_next_row(game_board, player_input);
                    game_board |= next_row;
                    player_board ^= next_row;
                    player_turn = false;
                } else {
                    println!("Invalid move.");
                    continue;
                }
            } else {
                let best_move = find_best_move(game_board, player_board, bot_board, depth);
                if best_move != None {
                    let best_move: usize = best_move.unwrap();
                    let next_row = get_next_row(game_board, best_move);
                    game_board |= next_row;
                    bot_board ^= next_row;
                    println!("Bot move: {}", best_move);
                    player_turn = true;
                } else {
                    println!("Failed to find the best move.");
                    process::exit(1);
                }
            }

            if has_won(player_board) {
                print_board(player_board, bot_board);
                println!("Player won!");
                player_turn = false;
                game_over = true;
            }

            if has_won(bot_board) {
                print_board(player_board, bot_board);
                println!("Bot won!");
                player_turn = true;
                game_over = true;
            }

            if is_board_full(game_board) {
                print_board(player_board, bot_board);
                println!("Draw");
                game_over = true;
            }
        } else {
            println!("Enter r to replay or q to quit:");
            let mut player_input: String = String::new();
            io::stdin()
                .read_line(&mut player_input)
                .expect("Failed to read line");

            let player_input: String = match player_input.trim().parse() {
                Ok(string) => string,
                Err(_) => {
                    println!("Failed to read input.");
                    continue;
                }
            };

            if player_input == "r" {
                game_board = 0;
                player_board = 0;
                bot_board = 0;
                game_over = false;
            } else if player_input == "q" {
                process::exit(1);
            }
        }
    }
}
