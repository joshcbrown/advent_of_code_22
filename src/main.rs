use std::{fs, process};

use clap::Parser;

mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Day of code to run
    day: u8,
    /// Path to data file for given day
    filename: String,
}

trait Solution {
    fn solve1(&self) -> String {
        "not yet implemented".into()
    }

    fn solve2(&self) -> String {
        "not yet implemented".into()
    }
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(args.filename).unwrap();
    let solution = match args.day {
        1 => day1::Day1 { content },
        26..=127 => {
            println!("input day too large");
            process::exit(1);
        }
        _ => {
            println!("not yet implemented");
            return;
        }
    };
    println!(
        "results of puzzle 1:\n{}\n\nresults of puzzle 2:\n{}",
        solution.solve1(),
        solution.solve2()
    );
}
