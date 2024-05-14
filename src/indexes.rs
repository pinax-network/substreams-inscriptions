use crate::helpers::{parse_data, parse_input, parse_mime_type, parse_value};
use crate::pb::inscriptions::types::v1::{Transaction, Transactions};
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};

#[substreams::handlers::map]
pub async fn map_transactions(block: Block) -> Result<Transactions, Error> {
    let mut transactions = vec![];

    for transaction in block.transactions() {
        // Transaction must be successful
        if transaction.status != TransactionTraceStatus::Succeeded as i32 {
            continue;
        }

        let value = parse_value(&transaction.value);
        let input = parse_input(&transaction.input);
        if input.len() == 0 {
            continue;
        }
        let data = parse_data(&input);
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
            mime_type: parse_mime_type(&input), // Mime type (ex: "application/json")
        });
    }

    log::debug!("Transactions: {:?}", transactions.len());
    Ok(Transactions {
        transactions,
    })
}

// #[substreams::handlers::map]
// fn index_events(events: Events) -> Result<Keys, Error> {
//     let mut keys = Keys::default();

//     events.events.into_iter().for_each(|e| {
//         if let Some(log) = e.log {
//             evt_keys(&log).into_iter().for_each(|k| {
//                 keys.keys.push(k);
//             });
//         }
//     });

//     Ok(keys)
// }