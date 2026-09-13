use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    #[serde(rename = "type")]
    pub record_type: TransactionType,

    #[serde(rename = "client")]
    pub record_client: u16,

    #[serde(rename = "tx")]
    pub record_tx: u32,

    #[serde(rename = "amount")]
    pub record_amount: Option<Decimal>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    #[serde(rename = "client")]
    pub record_client: u16,

    #[serde(rename = "available")]
    pub record_available: Decimal,

    #[serde(rename = "held")]
    pub record_held: Decimal,

    #[serde(rename = "total")]
    pub record_total: Decimal,

    #[serde(rename = "locked")]
    pub record_locked: bool,
}

#[derive(Debug)]
pub enum CommonError {
    MissingArgument(String),
    Input(csv::Error),
    Processing(String),
    Output(Box<dyn std::error::Error>),
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::MissingArgument(msg) => {
                write!(f, "missing argument error: {msg}")
            }
            CommonError::Input(err) => {
                write!(f, "input error: {err}")
            }
            CommonError::Processing(msg) => {
                write!(f, "processing error: {msg}")
            }
            CommonError::Output(err) => {
                write!(f, "output error: {err}")
            }
        }
    }
}
