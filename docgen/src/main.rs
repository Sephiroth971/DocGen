mod cli;
mod scanner;
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
