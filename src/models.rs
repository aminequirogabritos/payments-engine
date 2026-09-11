use rust_decimal::Decimal;
use serde::Deserialize;
use std::{env, error::Error, fs::File, io, process};

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
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
