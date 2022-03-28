mod counting;

fn main() {
    dynamic_programming();
    // let starting_game_state = GameState::new();
    
    // brute_force();
}

fn brute_force() {
    let starting_game_state = GameState::new();

    let average_score = dice_roll(starting_game_state);
    
    println!("Average score: {}", average_score);
}

fn dynamic_programming() {
    const TABLE_SIZE: usize = 7usize.pow(9);
    
    // 1.21 GB memory consumption
    let scores :Vec<[f32; 6]> = vec![[0.0; 6]; TABLE_SIZE];
    let moves :Vec<[u8; 6]> = vec![[0; 6]; TABLE_SIZE];
    
    // for board in counting::digit_numbers::<9,7>(1) {
    //     for empty_position in board
    //         .into_iter()
    //         .enumerate()
    //         .filter(|(_, value)| **value == 0)
    //         .map(|(position, _)| position) {

    //         }

    //     //println!("{:?} -> {:2}", val, counting::to_value::<4,3>(val))
    // }

}

fn score(board: &[u8; 9]) -> f64 {
    (1000
        - ((board[0] + board[1] + board[2]) as i32 * 100
            + (board[3] + board[4] + board[5]) as i32 * 10
            + (board[6] + board[7] + board[8]) as i32))
        .abs() as f64
}

fn player_choice(dice: i32, game_state: &GameState) -> f64 {
    let empty_positions = game_state.empty_positions();

    if empty_positions.len() == 1 {
        let next_state = game_state.play_move(dice, empty_positions[0]);
        return next_state.score();
    }

    let (_, best_score) = empty_positions
        .into_iter()
        .map(|position| {
            let next_state = game_state.play_move(dice, position);
            let expected_score = dice_roll(next_state);
            (position, expected_score)
        })
        .max_by(|(_, score0), (_, score1)| score0.partial_cmp(score1).unwrap())
        .unwrap();

    return best_score;
}

fn dice_roll(game_state: GameState) -> f64 {
    let score_sum: f64 = (1..=6).map(|dice| player_choice(dice, &game_state)).sum();
    return score_sum / 6.0;
}

#[derive(Clone)]
struct GameState {
    board: [i32; 9],
}

impl GameState {
    fn new() -> GameState {
        GameState { board: [0; 9] }
    }

    fn empty_positions(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter(|(_, value)| **value == 0)
            .map(|(position, _)| position)
            .collect()
    }

    fn play_move(&self, dice: i32, position: usize) -> GameState {
        let mut next_game_state = self.clone();
        next_game_state.board[position] = dice;

        next_game_state
    }

    fn score(&self) -> f64 {
        (1000
            - ((self.board[0] + self.board[1] + self.board[2]) * 100
                + (self.board[3] + self.board[4] + self.board[5]) * 10
                + (self.board[6] + self.board[7] + self.board[8])))
            .abs() as f64
    }
}
