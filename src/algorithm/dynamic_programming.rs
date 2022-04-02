use crate::counting;
use crate::game;

pub fn explicit() -> Vec<[u8; 6]> {
    const TABLE_SIZE: usize = 7usize.pow(9);

    // score[board_value] = best expected score attainable with this board (averaged over dice rolls)
    let mut scores: Vec<f32> = vec![0.0; TABLE_SIZE];

    // moves[board_value][dice_roll] = move (board position where the number is placed)
    // to get score[<board_value with dice_roll inserted at best position>] (traceback)
    let mut moves: Vec<[u8; 6]> = vec![[0; 6]; TABLE_SIZE];

    // Initialization: iterate of board with only a single empty position (=numbers with only a single 0)
    for mut board in counting::digit_numbers::<9, 7>(1) {
        // board is our current game board
        let board_index = counting::to_value::<9, 7>(&board);

        let mut score_sum = 0.0;

        for dice_roll in 1..=6 {
            // we roll the dice and get dice_roll
            let (empty_position, _) = board
                .iter()
                .enumerate()
                .find(|(_, value)| **value == 0)
                .unwrap();

            // with a single empty position there is only one possible move and we can calculate the score
            board[empty_position] = dice_roll;
            score_sum += game::score(&board);
            board[empty_position] = 0;

            moves[board_index][(dice_roll - 1) as usize] = empty_position as u8;
        }
        scores[board_index] = score_sum / 6.0;
    }

    // Recursion:
    for num_zeros in 2..=9 {
        for mut board in counting::digit_numbers::<9, 7>(num_zeros) {
            // board is our current game board
            let board_index = counting::to_value::<9, 7>(&board);

            let mut score_sum = 0.0;

            for dice_roll in 1..=6 {
                // we roll the dice and get dice_roll
                let (best_score, best_position) = IntoIterator::into_iter(board)
                    .enumerate()
                    .filter(|(_, value)| *value == 0)
                    .map(
                        // for each empty position of the board we fill it with the dice_roll and
                        // calculate the expected score recursively by looking at the table
                        // and afterwards set the position back to 0
                        |(empty_position, _)| {
                            board[empty_position] = dice_roll;
                            let board_index = counting::to_value::<9, 7>(&board);
                            board[empty_position] = 0;

                            (scores[board_index], empty_position)
                        },
                    )
                    .min_by(|(score0, _), (score1, _)| score0.partial_cmp(score1).unwrap())
                    .unwrap();

                score_sum += best_score;
                moves[board_index][(dice_roll - 1) as usize] = best_position as u8;
            }

            scores[board_index] = score_sum / 6.0;
        }
    }

    println!(
        "The expected score with expectation-optimizing play is {}.",
        scores[0]
    );

    // return traceback table
    moves
}

// pub fn implicit() {
//     todo!()
// }
