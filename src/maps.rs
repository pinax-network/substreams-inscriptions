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
        if calldata.len() == 0 {
            continue
        }

        operations.push(OperationEvent {
            from: "".to_string(),
            to: "".to_string(),
            value: "".to_string(),
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