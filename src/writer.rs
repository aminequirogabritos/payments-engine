use crate::models::{CommonError, OutputRecord};

pub fn write_csv(clients: Vec<OutputRecord>) -> Result<(), CommonError> {
    let stdout = std::io::stdout();
    let mut writer = csv::Writer::from_writer(stdout);

    for client in clients {
        writer
            .serialize(client)
            .map_err(|err| CommonError::Output(Box::new(err)))?;
    }

    writer
        .flush()
        .map_err(|err| CommonError::Output(Box::new(err)))?;

    Ok(())
}
