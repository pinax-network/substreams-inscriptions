use crate::helpers::{json_to_string, json_to_u64};
use crate::pb::inscriptions::types::v1::{DeployOp, TransferOp};
use crate::pb::inscriptions::types::v1::{Block as _Block, MintOp, Operations, OperationEvent, Transaction as _Transaction, operation_event::Operation};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::Block;
use std::str;

#[substreams::handlers::map]
pub fn map_operations(block: Block) -> Result<Operations, Error> {
    let mut operations = vec![];

    let _block = _Block {
        number: block.number,
        hash: Hex(&block.hash).to_string(),
        timestamp: block.timestamp().seconds,
        parent_hash: Hex(block.clone().header.unwrap().parent_hash).to_string()
    };

    for transaction in block.transactions() {
        // TO-DO: move to helpers.rs
        let value = if let Some(big_int) = &transaction.value {
            if Hex(&big_int.bytes).to_string().len() == 0 {
                String::from("0")
            } else {
                Hex(&big_int.bytes).to_string()
            }
        } else {
            String::from("0")
        };

        // TO-DO: move to helpers.rs
        // verify calldata value is valid UTF8
        let input = match str::from_utf8(&transaction.input) {
            Ok(vec) => vec.to_string(),
            Err(_e) => continue,
        };

        let _transaction = _Transaction {
            hash: Hex(&transaction.hash).to_string(),
            index: transaction.index,
            from: Hex(&transaction.from).to_string(),
            to: Hex(&transaction.to).to_string(),
            value,
            nonce: transaction.nonce,
            input: input.clone(),
        };

        // check if calldata is a data field
        if transaction.input.len() == 0 {
            continue
        }

        // TO-DO: move to helpers.rs
        if input.len() >= 4 {
            if &input[0..4] == "data" {
                //
            } else {
                continue
            };
        }

        // parse json
        let json_str = input.splitn(2, ',').nth(1).unwrap_or_default();
        let json_data = match serde_json::from_str(json_str) {
            Ok(data) => data,
            Err(_e) => continue,
        };

        let tick = json_to_string(&json_data, "tick");
        let op = json_to_string(&json_data, "op");
        let p = json_to_string(&json_data, "p");

        // mint
        if op == "mint" {
            let operation = MintOp {
                p: p.clone(),
                op,
                tick,
                amt: json_to_u64(&json_data, "amt").unwrap(),
            };

            operations.push(OperationEvent {
                block: Some(_block.clone()),
                transaction: Some(_transaction),
                operation: Some(Operation::Mint(operation)),
            });
            continue;
        }

        // transfer
        if op == "transfer" {
            let operation = TransferOp {
                p,
                op,
                tick,
                amt: json_to_u64(&json_data, "amt").unwrap(),
            };

            operations.push(OperationEvent {
                block: Some(_block.clone()),
                transaction: Some(_transaction),
                operation: Some(Operation::Transfer(operation)),
            });
            continue;
        }

        // deploy
        if op == "deploy" {
            let operation = DeployOp {
                p,
                op,
                tick,
                max: json_to_u64(&json_data, "max").unwrap(),
                lim: json_to_u64(&json_data, "lim").unwrap(),
            };

            operations.push(OperationEvent {
                block: Some(_block.clone()),
                transaction: Some(_transaction),
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