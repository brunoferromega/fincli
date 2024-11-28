use std::{error::Error, fs::File};

use domain::cli::{Cli, Commands};
use domain::transaction::{TRecord, Transaction};
use reqwest::header;

pub mod domain;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match &cli.commands {
        Commands::Submit => {
            let (records, _csv_file) = file_to_record()?;

            for record in records.into_iter() {
                let _ = submit_data(record);
            }

            Ok(())
        }

        Commands::Clean => clean_file(),

        _ => {
            let transaction = Transaction::from(cli);

            let data_record = TRecord::from(transaction);
            dbg!(&data_record);

            let t_records: Vec<TRecord> = append_data(data_record)?;

            write_csv(t_records)?;
            Ok(())
        }
    }
}

fn file_to_record() -> Result<(Vec<TRecord>, File), Box<dyn Error>> {
    let csv_file = match File::open("db.csv") {
        Ok(f) => f,
        Err(_) => File::create("db.csv")?,
    };

    let records = csv::Reader::from_reader(&csv_file)
        .into_deserialize()
        .map(|record| record.unwrap())
        .collect::<Vec<TRecord>>();

    Ok((records, csv_file))
}

fn append_data(data_record: TRecord) -> Result<Vec<TRecord>, Box<dyn Error>> {
    let file = match File::open("db.csv") {
        Ok(file) => file,
        Err(_) => File::create("db.csv")?,
    };

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

fn submit_data(record: TRecord) -> Result<(), Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::blocking::Client::new();
    let req_builder = client
        .post("http://localhost:8080/transactions")
        .headers(headers)
        .json(&record);

    let response = req_builder.send()?;

    let transc = response.json::<TRecord>()?;
    dbg!(transc);

    Ok(())
}

#[allow(dead_code)]
fn clean_file() -> Result<(), Box<dyn Error>> {
    let _ = File::create("db.csv");
    Ok(())
}
