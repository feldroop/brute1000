mod counting;

fn main() {
    let now = std::time::Instant::now();

    let moves = dynamic_programming();

    println!("Precomputing moves took {}s.", now.elapsed().as_secs());

    println!("Do you want to play a game (as the dice)? (y/n)");

    if user_decision_input() {
        play_game(&moves);
    }
}

fn dynamic_programming() -> Vec<[u8; 6]> {
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
            score_sum += score(&board);
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
                let (best_score, best_position) = IntoIterator::into_iter(board.clone())
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

/// Potz1000 game score
fn score(board: &[u8; 9]) -> f32 {
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
        base_score -= board[4] as f32;
    }

    if board[2] == board[4] && board[4] == board[6] {
        base_score -= board[4] as f32;
    }

    base_score
}

fn print_board(board: &[u8; 9]) {
    println!(
        "Current board:\n{} {} {}\n{} {} {}\n{} {} {}",
        board[0], board[3], board[6], board[1], board[4], board[7], board[2], board[5], board[8]
    )
}

/// lets user play a game of Potz1000 as the dice and the program chooses the optimal moves
fn play_game(moves: &Vec<[u8; 6]>) {
    let mut board = [0u8; 9];

    for _ in 0..9 {
        print_board(&board);

        println!("Please enter next dice roll: ");
        let dice_roll = user_dice_input();

        let best_position = moves[counting::to_value::<9, 7>(&board)][(dice_roll - 1) as usize];
        board[best_position as usize] = dice_roll;
    }

    print_board(&board);
    println!("Final score: {}", score(&board))
}

/// Repeatedly ask the user for a number btween 1 and 6
fn user_dice_input() -> u8 {
    let dice_roll = loop {
        let mut dice_roll = String::new();
        std::io::stdin()
            .read_line(&mut dice_roll)
            .expect("Error reading stdin");

        let dice_roll: u8 = match dice_roll.trim().parse::<u8>() {
            Ok(d) => d,
            Err(_) => {
                println!("Please enter a valid dice roll number.");
                continue;
            }
        };

        if dice_roll > 0 && dice_roll < 7 {
            break dice_roll;
        } else {
            println!("Numbers must be in the range of 1 to 6.");
        }
    };
    dice_roll
}

/// Repeatedly read stdin to get a 'y' or 'n' character and return true if 'y' was given
fn user_decision_input() -> bool {
    let answer = loop {
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Error reading stdin");

        answer = answer.trim().to_string();
        if answer == "y" || answer == "n" {
            break answer;
        }

        print!("Please enter 'y' or 'n'");
    };

    answer == "y"
}
