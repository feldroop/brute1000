use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(
    author = "Felix Droop",
    version = "0.1.0",
    about = "Optimal brute force precomputation AI for the game Potz1000",
    long_about = None
)]
pub struct Cli {
    /// What algorithm should be used for precomputing the game decisions
    #[clap(arg_enum, short, long, default_value_t = PrecomputeAlgorithm::ExplicitDynamicProgramming)]
    pub precompute_algorithm: PrecomputeAlgorithm,

    /// Whether definitely no game will be played after precomputation
    #[clap(short, long)]
    pub skip_game: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ArgEnum)]
pub enum PrecomputeAlgorithm {
    NaiveBruteForceRecursion,
    ExplicitDynamicProgramming,
    ImplicitDynamicProgramming,
}
