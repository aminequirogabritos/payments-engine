use rust_decimal::Decimal;
use serde::Deserialize;
use std::{env, error::Error, fs::File, io, process};

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    #[serde(rename = "type")]
    record_type: TransactionType,

    #[serde(rename = "client")]
    record_client: u16,

    #[serde(rename = "tx")]
    record_tx: u32,

    #[serde(rename = "amount")]
    record_amount: Decimal,
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

