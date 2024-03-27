use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use crate::pb::inscriptions::types::v1::{Operations, operation_event};

#[substreams::handlers::map]
pub fn graph_out(operations: Operations) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in operations.operations {
        let transaction = event.transaction.unwrap();
        let block = event.block.unwrap();
        let id = format!("{}-{}-{}", block.number, transaction.index, transaction.hash);

        let row = tables
            .create_row("Operation", id)
            // block information
            .set("block_hash", block.hash)
            .set("block_number", block.number)
            .set("block_timestamp", block.timestamp)
            .set("block_parent_hash", block.parent_hash)

            // trace information
            .set("transaction_hash", transaction.hash)
            .set("transaction_value", transaction.value)
            .set("transaction_index", transaction.index)
            .set("from", transaction.from)
            .set("to", transaction.to);

        // Operation specific fields
        match event.operation.unwrap() {
            operation_event::Operation::Mint(op) => {
                row
                    .set("p", op.p)
                    .set("op", op.op)
                    .set("tick", op.tick)
                    .set("amt", op.amt)
            },
            operation_event::Operation::Transfer(op) => {
                row
                    .set("p", op.p)
                    .set("op", op.op)
                    .set("tick", op.tick)
                    .set("amt", op.amt)
            },
            operation_event::Operation::Deploy(op) => {
                row
                    .set("p", op.p)
                    .set("op", op.op)
                    .set("tick", op.tick)
                    .set("max", op.max)
                    .set("lim", op.lim)
            }
        };
    }
    Ok(tables.to_entity_changes())
}
