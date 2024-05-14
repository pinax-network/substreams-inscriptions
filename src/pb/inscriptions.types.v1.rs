// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operations {
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<OperationEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationEvent {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(oneof="operation_event::Operation", tags="2, 3, 4")]
    pub operation: ::core::option::Option<operation_event::Operation>,
}
/// Nested message and enum types in `OperationEvent`.
pub mod operation_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="2")]
        Deploy(super::DeployOp),
        #[prost(message, tag="3")]
        Mint(super::MintOp),
        #[prost(message, tag="4")]
        Transfer(super::TransferOp),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
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
    #[prost(string, tag="8")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub mime_type: ::prost::alloc::string::String,
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
