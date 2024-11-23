use std::{error::Error, process};

use clap::Parser;
use fincli::{domain::cli::{Cli, Commands}, run};

fn main() {
    let args = Cli::parse();
    dbg!(&args);

    let result = match &args.commands {
        Commands::Submit => submit_data(),
        _ => run(args),
    };

    if let Err(error) = result {
        println!("{}", error);
        process::exit(1);
    }
}

fn submit_data() -> Result<(), Box<dyn Error>> {
    todo!("IMPLEMENT -> this function for load data from csv and send to the backend");
}
