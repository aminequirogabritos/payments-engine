
use crate::models::{CommonError, InputRecord};
use csv::Trim;
use serde::Deserialize;
use std::{env, error::Error, fs::File, io, process};

pub fn read_csv(file_name: &String) -> Result<Vec<InputRecord>, CommonError> {

    let file = File::open(file_name)
        .map_err(|err| CommonError::Input(csv::Error::from(err)))?;

    let mut rdr = csv::ReaderBuilder::new().trim(Trim::All).from_reader(file);

    rdr.deserialize()
        .collect::<Result<Vec<InputRecord>, _>>()
        .map_err(CommonError::Input)
}