use chrono::{DateTime, Utc};
use clap::{command, Parser};

#[allow(dead_code)]
#[derive(Debug)]
struct Transaction {
    name: String,
    amount: f32,
    date_time: DateTime<Utc>,
}

#[allow(dead_code)]
impl Transaction {
    fn new(name: &str, amount: f32) -> Self {
        Self {
            name: String::from(name),
            amount,
            date_time: Utc::now(),
        }
    }

    fn from(args: Args) -> Self {
        Self {
            name: args.name,
            amount: args.amount,
            date_time: Utc::now(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Financial cli app for register transactions", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    amount: f32,
}

pub fn run(args: Args) {
    let transaction = Transaction::from(args);
    println!("{:#?}", transaction);

    todo!("Format date (remove ms) AND save in a CSV file");
}
