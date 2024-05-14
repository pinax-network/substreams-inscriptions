use crate::helpers::{parse_data, bytes_to_utf8, parse_mime_type, parse_value};
use substreams::pb::sf::substreams::index::v1::Keys;
use crate::pb::inscriptions::types::v1::{Transaction, Transactions};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

#[substreams::handlers::map]
fn index_transactions(transactions: Transactions) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    if transactions.transactions.len() > 0 {
        keys.keys.push("inscriptions".to_string());
    }

    Ok(keys)
}

#[substreams::handlers::map]
pub async fn map_transactions(block: Block) -> Result<Transactions, Error> {
    let mut transactions = vec![];

    for transaction in block.transactions() {
        // Transaction must be successful
        if transaction.status != TransactionTraceStatus::Succeeded as i32 {
            continue;
        }
        // calldata
        if transaction.input.len() == 0 {
            continue;
        }
        let value = parse_value(&transaction.value); // ETH value
        let utf8 = bytes_to_utf8(&transaction.input);
        if utf8.is_none() {
            continue;
        }
        let mime_type = parse_mime_type(utf8.as_ref().unwrap()); // Mime type (ex: "application/json")
        if mime_type.is_none() {
            continue;
        }
        let data = parse_data(utf8.as_ref().unwrap());
        if data.is_none() {
            continue;
        }

        transactions.push(Transaction {
            hash: Hex(&transaction.hash).to_string(),
            index: transaction.index,
            from: Hex(&transaction.from).to_string(),
            to: Hex(&transaction.to).to_string(),
            value,
            nonce: transaction.nonce,
            input: Hex(&transaction.input).to_string(),
            data: data.unwrap().to_string(),
            mime_type: mime_type.unwrap().to_string(),
        });
    }

    log::debug!("Transactions: {:?}", transactions.len());
    Ok(Transactions {
        transactions,
    })
}
