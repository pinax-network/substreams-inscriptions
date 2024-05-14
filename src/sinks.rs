use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::{change::AsString, pb::database::{table_change, DatabaseChanges}};
use crate::{keys::to_key, pb::inscriptions::types::v1::{operation_event, Operations}};

#[substreams::handlers::map]
pub fn db_out(clock: Clock, operations: Operations) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in operations.operations {
        let transaction = event.transaction.unwrap();
        let key = to_key(&transaction);

        let row = tables
            .push_change("events", key, 0, table_change::Operation::Create)
            // block information
            .change("block_id", ("", clock.id.as_str()))
            .change("block_num", ("", clock.number.as_string().as_str()))
            .change("timestamp", ("", timestamp.as_str()))

            // transaction information
            .change("transaction_hash", ("", transaction.hash.as_str()))
            .change("transaction_value", ("", transaction.value.to_string().as_str()))
            .change("transaction_index", ("", transaction.index.to_string().as_str()))
            .change("from", ("", transaction.from.as_str()))
            .change("to", ("", transaction.to.as_str()));

        // Inscription operation specific fields
        match event.operation.unwrap() {
            operation_event::Operation::Mint(op) => {
                row.table = "mint_events".to_string();
                row
                    .change("p", ("", op.p.as_str()))
                    .change("tick", ("", op.tick.as_str()))
                    .change("op", ("", op.op.as_str()))
                    .change("amt", ("", op.amt.as_string().as_str()))
            },
            operation_event::Operation::Transfer(op) => {
                row.table = "transfer_events".to_string();
                row
                    .change("p", ("", op.p.as_str()))
                    .change("tick", ("", op.tick.as_str()))
                    .change("op", ("", op.op.as_str()))
                    .change("amt", ("", op.amt.as_string().as_str()))
            },
            operation_event::Operation::Deploy(op) => {
                row.table = "deploy_events".to_string();
                row
                    .change("p", ("", op.p.as_str()))
                    .change("tick", ("", op.tick.as_str()))
                    .change("op", ("", op.op.as_str()))
                    .change("max", ("", op.max.as_string().as_str()))
                    .change("lim", ("", op.lim.as_string().as_str()))
            }
        };
    }
    Ok(tables)
}
