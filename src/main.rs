mod counting;
mod algorithm;
mod game;
mod user;

fn main() {
    let now = std::time::Instant::now();

    let moves = algorithm::dynamic_programming::explicit();

    println!("Precomputing moves took {}s.", now.elapsed().as_secs());

    println!("Do you want to play a game (as the dice)? (y/n)");

    if user::decision_input() {
        user::play_game(&moves);
    }
}
