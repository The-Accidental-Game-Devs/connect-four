pub type Bitboard = u64;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;

pub fn get_top_mask(col: usize) -> Bitboard {
    let shift: usize = col * COLS + ROWS - 1;
    1 << shift
}

pub fn get_bottom_mask(col: usize) -> Bitboard {
    let shift: usize = col * COLS;
    1 << shift
}

pub fn get_col_mask(col: usize) -> Bitboard {
    let offset: usize = col * COLS;
    let bit: Bitboard = (1 << ROWS) - 1;
    bit << offset
}

pub fn get_next_row(bitboard: Bitboard, col: usize) -> Bitboard {
    let next_row: Bitboard = bitboard + get_bottom_mask(col);
    next_row & get_col_mask(col)
}

pub fn indices_from_bitmask(bitboard: Bitboard) -> Option<(usize, usize)> {
    let pos = bitboard.trailing_zeros() as usize;
    let row = pos % (ROWS + 1);
    let col = pos / (ROWS + 1);

    if row < ROWS {
        return Some((row, col));
    }
    None
}

pub fn can_place(bitboard: Bitboard, col: usize) -> bool {
    if col >= COLS {
        return false;
    }
    if (bitboard & get_top_mask(col)) > 0 {
        return false;
    }
    true
}

pub fn is_board_full(bitboard: Bitboard) -> bool {
    bitboard.count_ones() as usize >= ROWS * COLS
}

pub fn has_won(bitboard: Bitboard) -> bool {
    let horizontal = bitboard & (bitboard << (ROWS + 1));
    if horizontal & (horizontal << ((ROWS + 1) * 2)) > 0 {
        return true;
    }

    let vertical = bitboard & (bitboard << 1);
    if vertical & (vertical << 2) > 0 {
        return true;
    }

    let main_diagonal = bitboard & (bitboard << (ROWS + 2));
    if main_diagonal & (main_diagonal << ((ROWS + 2) * 2)) > 0 {
        return true;
    }

    let anti_diagonal = bitboard & (bitboard << ROWS);
    if anti_diagonal & (anti_diagonal << (ROWS * 2)) > 0 {
        return true;
    }

    false
}

pub fn is_game_over(
    game_board: Bitboard,
    player1_board: Bitboard,
    player2_board: Bitboard,
) -> bool {
    if has_won(player1_board) || has_won(player2_board) {
        return true;
    }
    if is_board_full(game_board) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_top_mask() {
        let result_a = get_top_mask(0);
        assert_eq!(result_a, 0b100000);

        let result_b = get_top_mask(1);
        assert_eq!(result_b, 0b100000_0_000000);
    }

    #[test]
    fn test_get_bottom_mask() {
        let result_a = get_bottom_mask(0);
        assert_eq!(result_a, 0b1);

        let result_b = get_bottom_mask(1);
        assert_eq!(result_b, 0b000001_0_000000);
    }

    #[test]
    fn test_get_col_mask() {
        let result_a = get_col_mask(0);
        assert_eq!(result_a, 0b111111);

        let result_b = get_col_mask(1);
        assert_eq!(result_b, 0b111111_0_000000);
    }

    #[test]
    fn test_get_next_row() {
        let bitboard_a: Bitboard = 0;
        let result_a = get_next_row(bitboard_a, 0);
        assert_eq!(result_a, 0b000001);

        let bitboard_b: Bitboard = 0b000001_0_000000;
        let result_b = get_next_row(bitboard_b, 1);
        assert_eq!(result_b, 0b000010_0_000000);
    }

    #[test]
    fn test_indices_from_bitmask() {
        let bitmask_a: Bitboard = 1;
        let result_a = indices_from_bitmask(bitmask_a);
        assert_eq!(Some((0, 0)), result_a);

        let bitmask_b: Bitboard = 0b100000_0_000000;
        let result_b = indices_from_bitmask(bitmask_b);
        assert_eq!(Some((5, 1)), result_b);

        let bitmask_c: Bitboard = 0b000000_1_000000;
        let result_c = indices_from_bitmask(bitmask_c);
        assert_eq!(None, result_c);
    }

    #[test]
    fn test_can_place() {
        let bitboard_a: Bitboard = 0;
        let result_a = can_place(bitboard_a, 0);
        assert_eq!(result_a, true);

        let bitboard_b: Bitboard = 0b111111_0_000000;
        let result_b = can_place(bitboard_b, 1);
        assert_eq!(result_b, false);
    }

    #[test]
    fn test_is_board_full() {
        let bitboard_a: Bitboard = 0;
        let result_a = is_board_full(bitboard_a);
        assert_eq!(result_a, false);

        let bitboard_b: Bitboard = 0b111111_0_111111_0_111111_0_111111_0_111111_0_111111_0_111111;
        let result_b = is_board_full(bitboard_b);
        assert_eq!(result_b, true);
    }

    #[test]
    fn test_has_won() {
        let no_win: Bitboard = 0;
        assert_eq!(has_won(no_win), false);

        let horizontal_win: Bitboard =
            0b000001_0_000001_0_000001_0_000001_0_000001_0_000001_0_000001;
        assert_eq!(has_won(horizontal_win), true);

        let vertical_win: Bitboard = 0b111111;
        assert_eq!(has_won(vertical_win), true);

        let main_diagonal_win: Bitboard = 0b001000_0_000100_0_000010_0_000001;
        assert_eq!(has_won(main_diagonal_win), true);

        let anti_diagonal_win: Bitboard = 0b000100_0_001000_0_010000_0_100000;
        assert_eq!(has_won(anti_diagonal_win), true);
    }
}
