use rust_decimal::Decimal;
use std::{fmt::Debug, sync::Arc};

use crate::models::{
    InputRecord, OutputRecord,
    TransactionType::{Chargeback, Deposit, Dispute, Resolve, Withdrawal},
};

use std::collections::HashMap;

pub fn process_payments(transactions: Vec<InputRecord>) {
    println!("hello from processor!");

    let mut clients: HashMap<u16, OutputRecord> = HashMap::new();

    for record in &transactions {
        let client_id = record.record_client;

        let client = clients.entry(client_id).or_insert_with(|| OutputRecord {
            record_client: client_id,
            record_available: Decimal::ZERO,
            record_held: Decimal::ZERO,
            record_total: Decimal::ZERO,
            record_locked: false,
        });

        if client.record_locked {
            println!("Client locked!");
            continue;
        }

        match record.record_type {
            Deposit => {
                println!(">> Transaction type: Deposit");
                if let Some(amount) = record.record_amount {
                    client.record_available += amount;
                    client.record_total += amount;
                    println!("After deposit: {:?}", client);
                }
            }

            Withdrawal => {
                println!(">> Transaction type: Withdrawal");
                if let Some(amount) = record.record_amount {
                    if client.record_available >= amount {
                        client.record_available -= amount;
                        client.record_total -= amount;
                        println!("After withdrawal: {:?}", client.record_available);
                    } else {
                        println!("Not enough funds!")
                    }
                }
            }

            Dispute => {
                println!(">> Transaction type: Dispute");
                if let Some(amount) = find_transaction_amount(&transactions, record.record_tx) {
                    client.record_available -= amount;
                    client.record_held += amount;

                    println!("After dispute: {:?}", client);
                }
            }

            Resolve => {
                // TODO : check if there was a dispute for this tx before a resolve
                println!(">> Transaction type: Resolve");
                if let Some(amount) = find_transaction_amount(&transactions, record.record_tx) {
                    client.record_held -= amount;
                    client.record_available += amount;

                    println!("After resolve: {:?}", client);
                }
            }

            Chargeback => {
                // TODO : check if there was a dispute for this tx before a chargeback
                println!(">> Transaction type: Chargeback");
                if let Some(amount) = find_transaction_amount(&transactions, record.record_tx) {
                    client.record_held -= amount;
                    client.record_total -= amount;
                    client.record_locked = true;

                    println!("After chargeback: {:?}", client);
                }
            }
        }
    }
}

fn find_transaction_amount(transactions: &[InputRecord], tx_id: u32) -> Option<Decimal> {
    transactions
        .iter()
        .find(|record| record.record_tx == tx_id)
        .and_then(|record| record.record_amount)
}
