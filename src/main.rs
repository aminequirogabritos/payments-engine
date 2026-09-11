use serde::Deserialize;
use std::{env, error::Error, fs::File, io, process};

#[derive(Debug, Deserialize)]
struct Record {
    user: u16,
}

fn read_csv(file: File) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(file);

    let records: Vec<Record> = rdr.deserialize().collect::<Result<Vec<Record>, _>>()?;

    Ok(records)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let file = File::open(file_name);

    let records = match file {
        Ok(file_result) => match read_csv(file_result) {
            Ok(records) => records,
            Err(err) => {
                println!("error running example: {}", err);
                process::exit(1);
            }
        },
        Err(file_err) => {
            println!("error running example: {}", file_err);
            process::exit(1);
        }
    };

    for record in &records {
        println!("{:?}", record);
    }

    /*
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    } */
}
