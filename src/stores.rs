use std::collections::HashMap;

use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};

use crate::pb::inscriptions::types::v1::{operation_event::{self, Operation}, Operations};

#[substreams::handlers::store]
pub fn store_balances(operations: Operations, store: StoreAddInt64) {
    for event in operations.operations {
        let transaction = event.transaction.unwrap();
        let operation = event.operation.unwrap();
        let tick = get_tick(operation.clone());
        let p = get_p(operation.clone());

        // Operation specific fields
        match operation.clone() {
            operation_event::Operation::Mint(op) => {
                // Validation
                // a.1. check if max supply is not reached
                // a.2. check if lim is not reached
                store.add(0, format!("{}-{}-{}", p, tick, transaction.from), op.amt);
            },
            operation_event::Operation::Transfer(op) => {
                store.add(0, format!("{}-{}-{}", p, tick, transaction.from), -op.amt);
                store.add(0, format!("{}-{}-{}", p, tick, transaction.to), op.amt)
            },
            operation_event::Operation::Deploy(_op) => {
                // no-op
            }
        };
    }
}

pub fn get_tick(operation: Operation) -> String {
    match operation {
        operation_event::Operation::Mint(op) => {
            op.tick
        },
        operation_event::Operation::Transfer(op) => {
            op.tick
        },
        operation_event::Operation::Deploy(op) => {
            op.tick
        }
    }
}

pub fn get_p(operation: Operation) -> String {
    match operation {
        operation_event::Operation::Mint(op) => {
            op.p
        },
        operation_event::Operation::Transfer(op) => {
            op.p
        },
        operation_event::Operation::Deploy(op) => {
            op.p
        }
    }
}

pub fn get_balance(balances: HashMap<String, i64>, key: String) -> i64 {
    match balances.get(&key) {
        // 1. check balance from in-memory balances
        Some(balance) => *balance,
        // 2. check balance from store
        // 3. if balance does not exist, return 0
        None => 0
    }
}
