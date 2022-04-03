use super::Output;
use crate::counting;
use crate::game;
use crate::game::{Board, Move, Tile};
use crate::game::{BOARD_SIZE, NUM_DICE_SIDES, NUM_GAME_STATES, NUM_TILE_VALUES};

pub fn naive() -> Output {
    let mut scores: Vec<f32> = vec![0.0; NUM_GAME_STATES];
    let mut moves: Vec<[Move; NUM_DICE_SIDES]> = vec![[0; NUM_DICE_SIDES]; NUM_GAME_STATES];

    let mut board: Board = [0; BOARD_SIZE];

    roll_dice(0, &mut board, &mut scores, &mut moves, false);

    Output { scores, moves }
}

pub fn with_cache() -> Output {
    let mut scores: Vec<f32> = vec![-1.0; NUM_GAME_STATES];
    let mut moves: Vec<[Move; NUM_DICE_SIDES]> = vec![[0; NUM_DICE_SIDES]; NUM_GAME_STATES];

    let mut board: Board = [0; BOARD_SIZE];

    roll_dice(0, &mut board, &mut scores, &mut moves, true);

    Output { scores, moves }
}

fn player_choice(
    dice_roll: u32,
    moves_played: u32,
    board: &mut Board,
    board_index: usize,
    scores: &mut Vec<f32>,
    moves: &mut Vec<[Move; NUM_DICE_SIDES]>,
    use_cache: bool
) -> f32 {
    if moves_played == 8 {
        let (empty_position, _) = board
            .iter()
            .enumerate()
            .find(|(_, value)| **value == 0)
            .unwrap();

        // with a single empty position there is only one possible move and we can calculate the score
        board[empty_position] = dice_roll as Tile;
        let score = game::score(board);
        board[empty_position] = 0;
        return score as f32;
    }

    let (best_score, best_position) = IntoIterator::into_iter(*board)
        .enumerate()
        .filter(|(_, value)| *value == 0)
        .map(
            // for each empty position of the board we fill it with the dice_roll and
            // calculate the expected score recursively by looking at the table
            // and afterwards set the position back to 0
            |(empty_position, _)| {
                board[empty_position] = dice_roll as Tile;
                let score = roll_dice(moves_played + 1, board, scores, moves, use_cache);
                board[empty_position] = 0;

                (score, empty_position)
            },
        )
        .min_by(|(score0, _), (score1, _)| score0.partial_cmp(score1).unwrap())
        .unwrap();

    moves[board_index][(dice_roll - 1) as usize] = best_position as Move;

    best_score
}

fn roll_dice(
    moves_played: u32,
    board: &mut Board,
    scores: &mut Vec<f32>,
    moves: &mut Vec<[Move; NUM_DICE_SIDES]>,
    use_cache: bool,
) -> f32 {
    let board_index = counting::to_value::<BOARD_SIZE, NUM_TILE_VALUES>(board);

    // if the value was already computed, just return the scores
    if use_cache && scores[board_index] != -1.0 {
        return scores[board_index];
    }

    let score_sum: f32 = (1..=6)
        .map(|dice| {
            player_choice(
                dice,
                moves_played,
                board,
                board_index,
                scores,
                moves,
                use_cache,
            )
        })
        .sum();
    let score_average = score_sum / NUM_DICE_SIDES as f32;

    scores[board_index] = score_average;

    score_average
}
