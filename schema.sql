-------------------------------------------------
-- Meta tables to store Substreams information --
-------------------------------------------------

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

-----------------------------------------------------------
-- Tables to store the raw events without any processing --
-----------------------------------------------------------

CREATE TABLE IF NOT EXISTS transfer_events
(
    id                  String,
    block_hash          String,
    block_number        UInt64,
    block_timestamp     UInt32,
    block_parent_hash   String,
    transaction_hash    String,
    transaction_value   UInt256,
    transaction_index   UInt32,
    from                String,
    to                  String,
    p                   String,
    tick                String,
    op                  String,
    amt                 Int64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

CREATE TABLE IF NOT EXISTS mint_events
(
    id                  String,
    block_hash          String,
    block_number        UInt64,
    block_timestamp     UInt32,
    block_parent_hash   String,
    transaction_hash    String,
    transaction_value   UInt256,
    transaction_index   UInt32,
    from                String,
    to                  String,
    p                   String,
    tick                String,
    op                  String,
    amt                 Int64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

CREATE TABLE IF NOT EXISTS deploy_events
(
    id                  String,
    block_hash          String,
    block_number        UInt64,
    block_timestamp     UInt32,
    block_parent_hash   String,
    transaction_hash    String,
    transaction_value   UInt256,
    transaction_index   UInt32,
    from                String,
    to                  String,
    p                   String,
    tick                String,
    op                  String,
    max                 Int64,
    lim                 Int64
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);
