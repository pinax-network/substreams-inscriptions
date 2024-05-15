use crate::helpers::{bytes_to_utf8, parse_data, parse_mime_type, parse_value};
use crate::pb::inscriptions::types::v1::{Transaction, Transactions};
use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

#[substreams::handlers::map]
fn index_transactions(transactions: Transactions) -> Result<Keys, Error> {
    Ok(match transactions.transactions.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["inscriptions".to_string()],
        },
    })
}

#[substreams::handlers::map]
pub async fn map_transactions(block: Block) -> Result<Transactions, Error> {
    let transactions = block
        .transactions()
        .filter_map(|transaction| {
            // Transaction must be successful
            if transaction.status != TransactionTraceStatus::Succeeded as i32 {
                return None;
            }
            // calldata
            if transaction.input.is_empty() {
                return None;
            }
            let value = match &transaction.value {
                Some(val) => parse_value(val),
                None => "0".to_string(),
            };
            let utf8 = match bytes_to_utf8(&transaction.input) {
                Some(utf8) => utf8,
                None => return None,
            };
            // Mime type (ex: "application/json")
            let mime_type = match parse_mime_type(utf8.as_str()) {
                Some(mime_type) => mime_type,
                None => return None,
            };
            let data = match parse_data(utf8.as_str()) {
                Some(data) => data,
                None => return None,
            };

            Some(Transaction {
                hash: Hex(&transaction.hash).to_string(),
                index: transaction.index,
                from: Hex(&transaction.from).to_string(),
                to: Hex(&transaction.to).to_string(),
                value,
                nonce: transaction.nonce,
                input: Hex(&transaction.input).to_string(),
                data: data.to_string(),
                mime_type: mime_type.to_string(),
            })
        })
        .collect::<Vec<_>>();

    log::debug!("Transactions: {:?}", transactions.len());
    Ok(Transactions { transactions })
}
