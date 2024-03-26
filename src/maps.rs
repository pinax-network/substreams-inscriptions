use crate::pb::inscriptions::types::v1::{Operations, OperationEvent};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::Block;
use serde_json::Value;
use std::str;
#[substreams::handlers::map]
pub fn map_operations(block: Block) -> Result<Operations, Error> {
    let mut operations = vec![];

    for transaction in block.transactions() {
        let trx = Hex(&transaction.hash).to_string();
        let source = Hex(&transaction.from).to_string();
        let destination = Hex(&transaction.to).to_string();
        let payload = if let Some(big_int) = &transaction.value {
            Hex(&big_int.bytes).to_string()
        } else {
            String::from("0")
        };

        //verify calldata value is valid UTF8

        let utf8_string = match str::from_utf8(&transaction.input) {
            Ok(vec) => vec.to_string(),
            Err(e) => continue,
        };

        //check if calldata is a data field

        if utf8_string.len() == 0 {
            continue
        }

        if utf8_string.len() >= 4 {
            if &utf8_string[0..4] == "data" {
                log::info!("Calldata length: {:?}", utf8_string.len());
            } else {
                continue
            };
        }

        //parse json

        let json_str = utf8_string.splitn(2, ',').nth(1).unwrap_or_default();
        let json_data: Value = serde_json::from_str(json_str).unwrap();

        //get elements from json data

        let p_value = if let Some(p_value) = json_data.get("p") {
            // Check if the "p" value is a string
            if let Some(p_str) = p_value.as_str() {
                Some(p_str)
            } else {
                None
            }
        } else {
            None
        };

        let op_value = if let Some(op_value) = json_data.get("op") {
            // Check if the "op" value is a string
            if let Some(op_str) = op_value.as_str() {
                Some(op_str)
            } else {
                None
            }
        } else {
            None
        };

        let tick_value = if let Some(tick_value) = json_data.get("tick") {
            // Check if the "tick" value is a string
            if let Some(tick_str) = tick_value.as_str() {
                Some(tick_str)
            } else {
                None
            }
        } else {
            None
        };

        let amt_value = if let Some(amt_value) = json_data.get("amt") {
            // Check if the "amt" value is a string
            if let Some(amt_str) = amt_value.as_str() {
                Some(amt_str)
            } else {
                None
            }
        } else {
            None
        };

        operations.push(OperationEvent {
            from: source,
            to: destination,
            value: payload,
            nonce: transaction.nonce,
            p: p_value.unwrap_or_default().to_string(),
            op: op_value.unwrap_or_default().to_string(),
            tick: tick_value.unwrap_or_default().to_string(),
            amt: amt_value.unwrap_or_default().to_string(),
            transaction: trx,
            block_index: transaction.index,
        })
    }
    //log::debug!("Operations: {:?}", operations.len());
    Ok(Operations {
        operations
    })
}