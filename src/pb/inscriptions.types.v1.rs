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
    /// user operation
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub calldata: ::prost::alloc::string::String,
    /// trace information
    #[prost(string, tag="5")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub block_index: u32,
    #[prost(uint64, tag="7")]
    pub nonce: u64,
}
// @@protoc_insertion_point(module)
