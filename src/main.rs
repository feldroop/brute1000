use clap::Parser;
use user::{Cli, PrecomputeAlgorithm};

mod algorithm;
mod counting;
mod game;
mod user;

fn main() {
    let args = Cli::parse();

    let now = std::time::Instant::now();

    let output = match args.precompute_algorithm {
        PrecomputeAlgorithm::ExplicitDynamicProgramming => algorithm::explicit_dynamic_programming(),
        PrecomputeAlgorithm::ImplicitDynamicProgramming => algorithm::implicit_dynamic_programming(),
        PrecomputeAlgorithm::NaiveBruteForceRecursion => algorithm::naive_brute_force()
    };

    println!(
        "The expected score with expectation-optimizing play is {}.",
        output.scores[0]
    );

    println!("Precomputing moves took {}s.", now.elapsed().as_secs());

    if !args.skip_game {
        println!("Do you want to play a game (as the dice)? (y/n)");
    
        if user::decision_input() {
            user::play_game(&output.moves);
        }
    } 
}
