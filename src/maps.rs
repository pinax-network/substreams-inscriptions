use crate::pb::inscriptions::types::v1::{Operations, OperationEvent};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_operations(block: Block) -> Result<Operations, Error> {
    let mut operations = vec![];

    for transaction in block.transactions() {
        let trx = Hex(&transaction.hash).to_string();
        let calldata = Hex(&transaction.input).to_string();
        calldata = calldata.
        let source = Hex(&transaction.from).to_string();
        let destination = Hex(&transaction.to).to_string();
        let payload = if let Some(big_int) = &transaction.value {
            print!("{:?}", &big_int.bytes);
            Hex(&big_int.bytes).to_string()
        } else {
            String::from("0")
        };
        if calldata.len() == 0 {
            continue
        }

        operations.push(OperationEvent {
            from: source,
            to: destination,
            value: payload,
            nonce: transaction.nonce,
            calldata,
            transaction: trx,
            block_index: transaction.index,
        })
    }
    log::debug!("Operations: {:?}", operations.len());
    Ok(Operations {
        operations
    })
}