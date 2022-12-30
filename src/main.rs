use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Day of code to run
    day: u8,
    /// Path to data file for given day
    filename: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
