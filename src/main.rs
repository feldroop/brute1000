mod counting;
mod algorithm;
mod game;
mod user;

fn main() {
    let now = std::time::Instant::now();
    
    let output = algorithm::dynamic_programming::explicit();

    println!(
        "The expected score with expectation-optimizing play is {}.",
        output.scores[0]
    );


    println!("Precomputing moves took {}s.", now.elapsed().as_secs());

    println!("Do you want to play a game (as the dice)? (y/n)");

    if user::decision_input() {
        user::play_game(&output.moves);
    }
}
