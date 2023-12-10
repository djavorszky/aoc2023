mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod prelude;

use clap::Parser;
use prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "Running Day {}",
        cli.day.map_or("latest".to_string(), |n| n.to_string())
    );
    match cli.day.unwrap_or(0) {
        5 => day5::run_day()?,
        6 => day6::run_day()?,
        7 => day7::run_day()?,
        8 => day8::run_day()?,
        _ => day9::run_day()?,
    }

    Ok(())
}
