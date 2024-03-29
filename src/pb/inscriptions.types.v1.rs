// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operations {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<OperationEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balances {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub balance: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationEvent {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<Block>,
    #[prost(message, optional, tag="2")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(oneof="operation_event::Operation", tags="3, 4, 5")]
    pub operation: ::core::option::Option<operation_event::Operation>,
}
/// Nested message and enum types in `OperationEvent`.
pub mod operation_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="3")]
        Deploy(super::DeployOp),
        #[prost(message, tag="4")]
        Mint(super::MintOp),
        #[prost(message, tag="5")]
        Transfer(super::TransferOp),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(uint64, tag="1")]
    pub number: u64,
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
    /// seconds
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub index: u32,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub value: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub nonce: u64,
    #[prost(string, tag="7")]
    pub input: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOp {
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tick: ::prost::alloc::string::String,
    /// transfer
    #[prost(string, tag="3")]
    pub op: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub amt: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintOp {
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tick: ::prost::alloc::string::String,
    /// mint
    #[prost(string, tag="3")]
    pub op: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub amt: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployOp {
    #[prost(string, tag="1")]
    pub p: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tick: ::prost::alloc::string::String,
    /// deploy
    #[prost(string, tag="3")]
    pub op: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub max: i64,
    #[prost(int64, tag="5")]
    pub lim: i64,
}
// @@protoc_insertion_point(module)
