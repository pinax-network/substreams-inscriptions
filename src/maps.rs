use crate::helpers::{json_to_i64, parse_value, get_mime_type, validate_data, validate_utf8};
use crate::pb::inscriptions::types::v1::{DeployOp, TransferOp};
use crate::pb::inscriptions::types::v1::{Block as _Block, MintOp, Operations, OperationEvent, Transaction as _Transaction, operation_event::Operation};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

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
        // Transaction must be successful
        if transaction.status != TransactionTraceStatus::Succeeded as i32 {
            continue;
        }

        let value = parse_value(&transaction.value);

        // Verify calldata value is valid UTF8
        let input = match validate_utf8(&transaction.input) {
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

        //Verify data is valid and fetch mime type

        let mime_data = get_mime_type(&input);
        let (media_type, mime_subtype, mime_type) = match mime_data {
            Ok(mime_data) => mime_data,
            Err(_e) => continue,
        };

        println!("Media Type: {}", media_type);
        println!("Mime Subtype: {}", mime_subtype);
        println!("Mime Type: {}", mime_type);

        // Validate data

        let data = validate_data(input);
        let (p, op, tick, json_data) = match data {
            Ok(data) => data,
            Err(_e) => continue,
        };
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
                block: Some(_block.clone()),
                transaction: Some(_transaction),
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
                block: Some(_block.clone()),
                transaction: Some(_transaction),
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
