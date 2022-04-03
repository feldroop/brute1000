use crate::counting;
use crate::game;
use crate::game::{Move, Tile, Board};
use crate::game::{BOARD_SIZE, NUM_DICE_SIDES, NUM_TILE_VALUES};

/// lets user play a game of Potz1000 as the dice and the program chooses the optimal moves
pub fn play_game(moves: &[[Move; NUM_DICE_SIDES]]) {
    let mut board: Board = [0; BOARD_SIZE];

    for _ in 0..BOARD_SIZE {
        print_board(&board);

        println!("Please enter next dice roll: ");
        let dice_roll = dice_input();

        let best_position = moves[counting::to_value::<BOARD_SIZE, NUM_TILE_VALUES>(&board)]
            [(dice_roll - 1) as usize];
        board[best_position as usize] = dice_roll;
    }

    print_board(&board);
    println!("Final score: {}", game::score(&board))
}

/// Repeatedly ask the user for a number between 1 and 6
fn dice_input() -> Tile {
    let dice_roll = loop {
        let mut dice_roll = String::new();
        std::io::stdin()
            .read_line(&mut dice_roll)
            .expect("Error reading stdin");

        let dice_roll: Tile = match dice_roll.trim().parse::<Tile>() {
            Ok(d) => d,
            Err(_) => {
                println!("Please enter a valid dice roll number.");
                continue;
            }
        };

        if dice_roll > 0 && dice_roll <= (NUM_DICE_SIDES as u32) {
            break dice_roll;
        } else {
            println!("Numbers must be in the range of 1 to 6.");
        }
    };
    dice_roll
}

/// Repeatedly read stdin to get a 'y' or 'n' character and return true if 'y' was given
pub fn decision_input() -> bool {
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

fn print_board(board: &Board) {
    if BOARD_SIZE == 9 {
        println!(
            "Current board:\n{} {} {}\n{} {} {}\n{} {} {}",
            board[0],
            board[3],
            board[6],
            board[1],
            board[4],
            board[7],
            board[2],
            board[5],
            board[8]
        )
    } else {
        println!("Current board:\n{:?}", board)
    }
}
