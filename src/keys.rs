use crate::pb::inscriptions::types::v1::Transaction;

pub fn to_key(transaction: &Transaction) -> String {
    format!("{}-{}", transaction.hash, transaction.index)
}
