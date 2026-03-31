mod cli;
mod scanners;
//mod parser;
//mod generator;
//mod git;
mod utils;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
