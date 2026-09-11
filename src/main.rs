use std::{env, error::Error, fs::File, io, process};

pub mod models;
pub mod reader;
pub mod processor;
pub mod writer;

use crate::reader::read_csv;
use crate::processor::process_payments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let file = File::open(file_name);

    let records = match file {
        Ok(file_result) => match read_csv(file_result) {
            Ok(records) => records,
            Err(err) => {
                println!("error reading CSV: {}", err);
                process::exit(1);
            }
        },
        Err(file_err) => {
            println!("error opening file: {}", file_err);
            process::exit(1);
        }
    };

    for record in &records {
        println!("{:?}", record);
    }


}
