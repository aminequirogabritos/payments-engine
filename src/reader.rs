
use crate::models::InputRecord;
use csv::Trim;
use std::{env, error::Error, fs::File, io, process};


pub fn read_csv(file: File) -> Result<Vec<InputRecord>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().trim(Trim::All).from_reader(file);

    let records: Vec<InputRecord> = rdr.deserialize().collect::<Result<Vec<InputRecord>, _>>()?;
    println!("{:?}", records[0]);
    Ok(records)
}