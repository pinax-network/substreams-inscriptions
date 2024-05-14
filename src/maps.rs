use crate::helpers::{json_to_i64, json_to_string};
use crate::pb::inscriptions::types::v1::{
    operation_event::Operation, DeployOp, MintOp, OperationEvent, Operations, Transactions,
    TransferOp,
};
use substreams::{errors::Error, log};

#[substreams::handlers::map]
pub async fn map_operations(transactions: Transactions) -> Result<Operations, Error> {
    let operations = transactions
        .transactions
        .into_iter()
        .filter_map(|transaction| {
            let json_data = match serde_json::from_str(&transaction.data) {
                Ok(data) => data,
                Err(_) => return None,
            };

            let tick = json_to_string(&json_data, "tick");
            let op = json_to_string(&json_data, "op");
            let p = json_to_string(&json_data, "p");

            log::debug!("Data: {}", transaction.data);

            match op.as_str() {
                "mint" => {
                    let amt = match json_to_i64(&json_data, "amt") {
                        Some(amt) => amt,
                        None => return None,
                    };

                    let operation = MintOp { p, op, tick, amt };

                    Some(OperationEvent {
                        transaction: Some(transaction),
                        operation: Some(Operation::Mint(operation)),
                    })
                }
                "transfer" => {
                    let amt = match json_to_i64(&json_data, "amt") {
                        Some(amt) => amt,
                        None => return None,
                    };
                    let operation = TransferOp { p, op, tick, amt };

                    Some(OperationEvent {
                        transaction: Some(transaction),
                        operation: Some(Operation::Transfer(operation)),
                    })
                }
                "deploy" => {
                    let max = match json_to_i64(&json_data, "max") {
                        Some(max) => max,
                        None => return None,
                    };
                    let lim = match json_to_i64(&json_data, "lim") {
                        Some(lim) => lim,
                        None => return None,
                    };
                    let operation = DeployOp {
                        p,
                        op,
                        tick,
                        max,
                        lim,
                    };

                    Some(OperationEvent {
                        transaction: Some(transaction),
                        operation: Some(Operation::Deploy(operation)),
                    })
                }
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    log::debug!("Operations: {:?}", operations.len());
    Ok(Operations { operations })
}
