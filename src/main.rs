use clap::{Parser, Subcommand};

pub mod days;

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

    match args.command {
        Commands::Day { day } => {
            println!("Executing day {}...", day);
            match day {
                1 => days::day01::main(),
                x => println!("No code for day {} yet.", x),
            }
            println!("Done! 🎄")
        }
    };
}
