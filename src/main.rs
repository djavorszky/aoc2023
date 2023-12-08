mod day5;
mod day6;
mod day7;
mod prelude;

use clap::Parser;
use prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: u8,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Running Day {}", cli.day);
    match cli.day {
        5 => day5::run_day()?,
        6 => day6::run_day()?,
        7 => day7::run_day()?,
        _ => println!("Day {} not yet implemented", cli.day),
    }

    Ok(())
}
