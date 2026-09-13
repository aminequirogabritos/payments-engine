use std::env;

pub mod models;
pub mod processor;
pub mod reader;
pub mod writer;

use crate::models::CommonError;
use crate::processor::process_payments;
use crate::reader::read_csv;
use crate::writer::write_csv;

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), CommonError> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    let records = read_csv(file_name)?;

    let clients = process_payments(records)?;

    write_csv(clients)?;

    Ok(())
}
