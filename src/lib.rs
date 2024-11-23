use std::{error::Error, fs::File};

use domain::cli::Cli;
use domain::transaction::{TRecord, Transaction};

pub mod domain;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let transaction = Transaction::from(cli);

    let data_record = TRecord::from(transaction);
    dbg!(&data_record);

    let t_records: Vec<TRecord> = append_data(data_record)?;

    write_csv(t_records)?;

    Ok(())
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

#[allow(dead_code)]
fn submit_data(record: TRecord) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get("http://127.0.0.1:8080/")?;

    let client = reqwest::blocking::Client::new();
    let req_builder = client.post("http://127.0.0.1:8080/transactions");

    if response.status().is_success() {
        let post_resp = req_builder.json(&record).send()?;

        let transc = post_resp.json::<TRecord>()?;
        dbg!(transc);
    }

    let msg = response.text()?;
    println!("Is alive: {}", msg);

    Ok(())
}
