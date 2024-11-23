use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Financial cli app for register transactions", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Deposit {
        #[arg(long, short)]
        amount: f32,
        #[arg(long, short)]
        title: String,
        #[arg(long, short)]
        description: Option<String>,
    },

    Withdraw {
        #[arg(long, short)]
        amount: f32,
        #[arg(long, short)]
        title: String,
        #[arg(long, short)]
        description: Option<String>,
    },

    Submit,
}
