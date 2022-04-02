pub mod dynamic_programming;
pub mod naive;

use crate::game::{Move, NUM_DICE_SIDES};

#[non_exhaustive]
pub struct Output {
    /// scores[board_value] = best expected score attainable with the respective board
    /// (averaged over dice rolls)
    pub scores: Vec<f32>,

    // moves[board_value][dice_roll] = move (board position where the number is placed)
    // to get score[<board_value with dice_roll inserted at best position>]
    pub moves: Vec<[Move; NUM_DICE_SIDES]>,
}
