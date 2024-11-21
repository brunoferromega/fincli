use std::{error::Error, fs::File};

use domain::transaction::{TRecord, Transaction};
use domain::cli::Cli;

pub mod domain;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let transaction = Transaction::from(cli);

    let data_trecord = TRecord::from(transaction);

    dbg!(&data_trecord);

    let t_records: Vec<TRecord> = append_data(data_trecord)?;

    write_csv(t_records)?;

    todo!("FEAT -> Implement backend communication");
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
