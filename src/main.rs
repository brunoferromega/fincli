use std::process;

use clap::Parser;
use fincli::{domain::cli::Cli, run};

fn main() {
    let args = Cli::parse();
    dbg!(&args);

    if let Err(error) = run(args) {
        println!("{}", error);
        process::exit(1);
    }

    todo!("REFACTOR -> improve code readbility"); 
}

