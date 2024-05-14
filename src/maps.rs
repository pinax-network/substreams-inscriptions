use crate::helpers::{json_to_i64, json_to_string};
use crate::pb::inscriptions::types::v1::{DeployOp, TransferOp};
use crate::pb::inscriptions::types::v1::{MintOp, Operations, OperationEvent, Transactions, operation_event::Operation};
use substreams::errors::Error;
use substreams::log;

#[substreams::handlers::map]
pub async fn map_operations(transactions: Transactions) -> Result<Operations, Error> {
    let mut operations = vec![];

    for transaction in transactions.transactions {
        // Parse JSON data
        let json_data = match serde_json::from_str(&transaction.data) {
            Ok(data) => data,
            Err(_e) => continue,
        };

        let tick = json_to_string(&json_data, "tick");
        let op = json_to_string(&json_data, "op");
        let p = json_to_string(&json_data, "p");

        log::debug!("Data: {}", transaction.data);

        // mint
        if op == "mint" {
            let amt = json_to_i64(&json_data, "amt");
            if amt.is_none() {
                continue;
            }

            let operation = MintOp {
                p: p.clone(),
                op,
                tick,
                amt: amt.unwrap(),
            };

            operations.push(OperationEvent {
                transaction: Some(transaction),
                operation: Some(Operation::Mint(operation)),
            });
            continue;
        }

        // transfer
        if op == "transfer" {
            let amt = json_to_i64(&json_data, "amt");
            if amt.is_none() {
                continue;
            }
            let operation = TransferOp {
                p,
                op,
                tick,
                amt: amt.unwrap(),
            };

            operations.push(OperationEvent {
                transaction: Some(transaction),
                operation: Some(Operation::Transfer(operation)),
            });
            continue;
        }

        // deploy
        if op == "deploy" {
            let max = json_to_i64(&json_data, "max");
            let lim = json_to_i64(&json_data, "lim");
            if max.is_none() || lim.is_none() {
                continue;
            }
            let operation = DeployOp {
                p,
                op,
                tick,
                max: max.unwrap(),
                lim: lim.unwrap(),
            };

            operations.push(OperationEvent {
                transaction: Some(transaction),
                operation: Some(Operation::Deploy(operation)),
            });
            continue;
        }
    }

    log::debug!("Operations: {:?}", operations.len());
    Ok(Operations {
        operations
    })
}
