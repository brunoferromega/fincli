use std::{error::Error, fs::File};

use chrono::{DateTime, Utc};
use clap::{command, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Financial cli app for register transactions", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    amount: f32,
}

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

    fn from(cli: Cli) -> Self {
        Self {
            name: cli.name,
            amount: cli.amount,
            date_time: Utc::now(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct TRecord {
    name: String,
    amount: f32,
    date_time: String,
}

impl TRecord {
    fn from(transaction: Transaction) -> Self {
        Self {
            name: transaction.name,
            amount: transaction.amount,
            date_time: format!("{}", transaction.date_time.format("%d/%m/%Y %H:%M")),
        }
    }
}

pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    let transaction = Transaction::from(args);
    let data_trecord = TRecord::from(transaction);

    dbg!(&data_trecord);

    let t_records: Vec<TRecord> = append_data(data_trecord)?;

    write_csv(t_records)?;

    todo!("FEAT -> implement negavite valeu or add type column");
}

fn append_data(data_record: TRecord) -> Result<Vec<TRecord>, Box<dyn Error>> {
    let file = File::open("db.csv")?;
    let reader = csv::Reader::from_reader(file);

    let mut records: Vec<TRecord> = reader.into_deserialize().map(|r| r.unwrap()).collect();
    records.push(data_record);

    Ok(records)
}

fn write_csv(records: Vec<TRecord>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("db.csv")?;

    records
        .into_iter()
        .for_each(|record: TRecord| writer.serialize(record).unwrap());

    writer.flush()?;
    Ok(())
}
