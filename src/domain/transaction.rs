use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::cli::{Cli, Commands};


#[allow(dead_code)]
#[derive(Debug)]
pub struct Transaction {
    title: String,
    amount: f32,
    date_time: DateTime<Utc>,
}

#[allow(dead_code)]
impl Transaction {
    fn new(name: &str, amount: f32) -> Self {
        Self {
            title: String::from(name),
            amount,
            date_time: Utc::now(),
        }
    }

}

impl From<Cli> for Transaction {
    fn from(cli: Cli) -> Self {
        match cli.commands {
            Commands::Deposit { amount, title, .. } => Self {
                title,
                amount,
                date_time: Utc::now(),
            },

            Commands::Withdraw { amount, title, .. } => Self {
                title,
                amount,
                date_time: Utc::now(),
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TRecord {
    name: String,
    amount: f32,
    date_time: String,
}

impl From<Transaction> for TRecord {
    fn from(transaction: Transaction) -> Self {
        Self {
            name: transaction.title,
            amount: transaction.amount,
            date_time: format!("{}", transaction.date_time.format("%d/%m/%Y %H:%M")),
        }
    }
}