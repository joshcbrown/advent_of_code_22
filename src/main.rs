use std::fs;

use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Day of code to run
    day: u8,
    /// Path to data file for given day
    filename: String,
}

trait Solution {
    fn new(content: String) -> Self;

    fn solve1(&self) -> String {
        "not yet implemented".into()
    }

    fn solve2(&self) -> String {
        "not yet implemented".into()
    }

    fn run_day(&self) {
        println!(
            "results of puzzle 1:\n{}\n\nresults of puzzle 2:\n{}",
            self.solve1(),
            self.solve2()
        );
    }
}

fn main() {
    let args = Args::parse();
    let content = fs::read_to_string(args.filename).unwrap();
    match args.day {
        1 => day1::Day1::new(content).run_day(),
        2 => day2::Day2::new(content).run_day(),
        3 => day3::Day3::new(content).run_day(),
        4 => day4::Day4::new(content).run_day(),
        5 => day5::Day5::new(content).run_day(),
        6 => day6::Day6::new(content).run_day(),
        7 => day7::Day7::new(content).run_day(),
        8 => day8::Day8::new(content).run_day(),
        9 => day9::Day9::new(content).run_day(),
        10..=25 => todo!("haven't added functionality for this day"),
        _ => println!("day number too large"),
    }
}
