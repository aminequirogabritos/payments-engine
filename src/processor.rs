use rust_decimal::Decimal;

use crate::models::{
    CommonError, InputRecord, OutputRecord,
    TransactionType::{Chargeback, Deposit, Dispute, Resolve, Withdrawal},
};
use std::{env, error::Error, fs::File, io, process};

use std::collections::HashMap;

pub fn process_payments(transactions: Vec<InputRecord>) -> Result<Vec<OutputRecord>, CommonError> {

    let mut clients: HashMap<u16, OutputRecord> = HashMap::new();

    for (index, record) in transactions.iter().enumerate() {
        let client_id = record.record_client;

        let client = clients.entry(client_id).or_insert_with(|| OutputRecord {
            record_client: client_id,
            record_available: Decimal::ZERO,
            record_held: Decimal::ZERO,
            record_total: Decimal::ZERO,
            record_locked: false,
        });

        if client.record_locked {
            continue;
        }

        match record.record_type {
            Deposit => {
                if let Some(amount) = record.record_amount {
                    client.record_available += amount;
                    client.record_total += amount;
                } else {
                    return Err(CommonError::Processing(String::from(
                        "couldn't process input - no deposit amount",
                    )));
                }
            }

            Withdrawal => {
                if let Some(amount) = record.record_amount {
                    if client.record_available >= amount {
                        client.record_available -= amount;
                        client.record_total -= amount;
                    }
                } else {
                    return Err(CommonError::Processing(String::from(
                        "couldn't process input - no withdrawal amount",
                    )));
                }
            }

            Dispute => {
                if let Some(transaction) = find_transaction(&transactions, record.record_tx) {
                    if transaction.record_client == record.record_client
                        && transaction.record_type == Deposit
                    {
                        if let Some(amount) = transaction.record_amount {
                            client.record_available -= amount;
                            client.record_held += amount;
                        }
                    }
                }
            }

            Resolve => {
                if is_under_dispute(&transactions, index, record.record_tx) {
                    if let Some(transaction) = find_transaction(&transactions, record.record_tx) {
                        if transaction.record_client == record.record_client
                            && transaction.record_type == Deposit
                        {
                            if let Some(amount) = transaction.record_amount {
                                client.record_available += amount;
                                client.record_held -= amount;
                            }
                        }
                    }
                }
            }

            Chargeback => {
                if is_under_dispute(&transactions, index, record.record_tx) {
                    if let Some(transaction) = find_transaction(&transactions, record.record_tx) {
                        if transaction.record_client == record.record_client
                            && transaction.record_type == Deposit
                        {
                            if let Some(amount) = transaction.record_amount {
                                client.record_held -= amount;
                                client.record_total -= amount;
                                client.record_locked = true;
                            }
                        }
                    }
                }
            }
        }
    }

    let output_records: Vec<OutputRecord> = clients.into_values().collect();

    Ok(output_records)
}

fn find_transaction(transactions: &[InputRecord], tx_id: u32) -> Option<&InputRecord> {
    transactions.iter().find(|record| record.record_tx == tx_id)
}

fn is_under_dispute(transactions: &[InputRecord], current_index: usize, tx_id: u32) -> bool {
    transactions[..current_index]
        .iter()
        .any(|record| record.record_tx == tx_id && record.record_type == Dispute)
}