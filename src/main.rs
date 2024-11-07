use clap::Parser;

use fincli::{run, Args};

fn main() {
    let args = Args::parse();
    dbg!(&args);
    run(args);
}
