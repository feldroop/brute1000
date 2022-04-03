pub const BOARD_SIZE: usize = 9;
pub const NUM_DICE_SIDES: usize = 6;
pub const NUM_TILE_VALUES: usize = NUM_DICE_SIDES + 1;
pub const NUM_GAME_STATES: usize = NUM_TILE_VALUES.pow(BOARD_SIZE as u32);

// this must be able to fit a value of up to BOARD_SIZE - 1;
pub type Move = u8;
pub type Tile = crate::counting::Digit;
pub type Board = [Tile; BOARD_SIZE];

/// Potz1000 game score
pub fn score(board: &Board) -> f32 {
    if BOARD_SIZE == 9 {
        // board indices are interpreted as the following real board layout:
        // 0 3 6
        // 1 4 7
        // 2 5 8
        let mut base_score = (1000
            - ((board[0] + board[1] + board[2]) as i32 * 100
                + (board[3] + board[4] + board[5]) as i32 * 10
                + (board[6] + board[7] + board[8]) as i32))
            .abs() as f32;

        // bonus points for matching diagonals
        if board[0] == board[4] && board[4] == board[8] {
            base_score -= (board[4] * 10) as f32;
        }

        if board[2] == board[4] && board[4] == board[6] {
            base_score -= (board[4] * 10) as f32;
        }

        base_score
    } else {
        unimplemented!()
    }
}
