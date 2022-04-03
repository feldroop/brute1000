use super::Output;
use crate::counting;
use crate::game;
use crate::game::{Move, Tile};
use crate::game::{BOARD_SIZE, NUM_DICE_SIDES, NUM_GAME_STATES, NUM_TILE_VALUES};

pub fn explicit_dynamic_programming() -> Output {
    let mut scores: Vec<f32> = vec![0.0; NUM_GAME_STATES];

    let mut moves: Vec<[Move; NUM_DICE_SIDES]> = vec![[0; NUM_DICE_SIDES]; NUM_GAME_STATES];

    // Initialization: iterate of board with only a single empty position (=numbers with only a single 0)
    for mut board in counting::digit_numbers::<BOARD_SIZE, NUM_TILE_VALUES>(1) {
        // board is our current game board
        let board_index = counting::to_value::<BOARD_SIZE, NUM_TILE_VALUES>(&board);

        let mut score_sum = 0.0;

        for dice_roll in 1..=NUM_DICE_SIDES {
            // we roll the dice and get dice_roll
            let (empty_position, _) = board
                .iter()
                .enumerate()
                .find(|(_, value)| **value == 0)
                .unwrap();

            // with a single empty position there is only one possible move and we can calculate the score
            board[empty_position] = dice_roll as Tile;
            score_sum += game::score(&board);
            board[empty_position] = 0;

            moves[board_index][(dice_roll - 1) as usize] = empty_position as Move;
        }
        scores[board_index] = score_sum / (NUM_DICE_SIDES as f32);
    }

    // Recursion:
    for num_zeros in 2..=BOARD_SIZE {
        for mut board in counting::digit_numbers::<BOARD_SIZE, NUM_TILE_VALUES>(num_zeros) {
            // board is our current game board
            let board_index = counting::to_value::<BOARD_SIZE, NUM_TILE_VALUES>(&board);

            let mut score_sum = 0.0;

            for dice_roll in 1..=NUM_DICE_SIDES {
                // we roll the dice and get dice_roll
                let (best_score, best_position) = IntoIterator::into_iter(board)
                    .enumerate()
                    .filter(|(_, value)| *value == 0)
                    .map(
                        // for each empty position of the board we fill it with the dice_roll and
                        // calculate the expected score recursively by looking at the table
                        // and afterwards set the position back to 0
                        |(empty_position, _)| {
                            board[empty_position] = dice_roll as Tile;
                            let board_index =
                                counting::to_value::<BOARD_SIZE, NUM_TILE_VALUES>(&board);
                            board[empty_position] = 0;

                            (scores[board_index], empty_position)
                        },
                    )
                    .min_by(|(score0, _), (score1, _)| score0.partial_cmp(score1).unwrap())
                    .unwrap();

                score_sum += best_score;
                moves[board_index][(dice_roll - 1) as usize] = best_position as Move;
            }

            scores[board_index] = score_sum / (NUM_DICE_SIDES as f32);
        }
    }

    // return traceback table
    Output { moves, scores }
}
