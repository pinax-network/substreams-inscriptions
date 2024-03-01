use substreams::errors::Error;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

use crate::pb::inscriptions::types::v1::Operations;

#[substreams::handlers::map]
pub fn graph_out(operations: Operations) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    for event in operations.operations {
        let id = format!("{}-{}", event.block_index, event.transaction);

        tables
            .create_row("Operations", id)
            // event payload
            .set("from", event.from)
            .set("to", event.to)
            .set("value", event.value)
            // // trace information
            .set("transaction", event.transaction);
    }
    Ok(tables.to_entity_changes())
}
