use aoc24::days;
use clap::{Parser, Subcommand};
use std::time::Instant;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "aoc")]
#[command(
    about = "Advent of Code 2024",
    long_about = "Executing a command for the Advent of Code 2024! 🎄"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Day { day: u16 },
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    let start = Instant::now();
    match args.command {
        Commands::Day { day } => match day {
            1 => days::day01::main(),
            x => log::error!("No code for day {} yet.", x),
        },
    };
    let duration = start.elapsed();
    log::info!("Done! 🎄 -- Elapsed time: {:?}", duration);
}
