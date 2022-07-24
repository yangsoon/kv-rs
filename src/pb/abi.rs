/// 来自客户端的命令请求
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(
        oneof = "command_request::RequestData",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag = "1")]
        Hget(super::Hget),
        #[prost(message, tag = "2")]
        Hmget(super::Hmget),
        #[prost(message, tag = "3")]
        Hset(super::Hset),
        #[prost(message, tag = "4")]
        Hmset(super::Hmset),
        #[prost(message, tag = "5")]
        Hdel(super::Hdel),
        #[prost(message, tag = "6")]
        Hmdel(super::Hmdel),
        #[prost(message, tag = "7")]
        Hexit(super::Hexit),
        #[prost(message, tag = "8")]
        Hmexit(super::Hmexit),
        #[prost(message, tag = "9")]
        Hgetall(super::Hgetall),
    }
}
/// 服务器的响应
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    /// 状态码；复用 HTTP 2xx/4xx/5xx 状态码
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// 如果不是 2xx, message里包含详细信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 成功返回的values
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// 成功返回的Kvpairs
    #[prost(message, repeated, tag = "4")]
    pub pairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// 从 table 取一个 key，返回 value
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 从 table 中获取所有的 Kvpair
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hgetall {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
}
/// 从 table 中获取一组 keys 返回它们的值
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hmget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 往table里面存储一个kvpair
/// 如果 table 不存在就创建这个table
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pair: ::core::option::Option<Kvpair>,
}
/// 往table里面存储多个pairs
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hmset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// 从table里删除一个值，返回它之前的值
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 从table里删除多个值，返回它们之前的值
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hmdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub key: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 查看key是否存在
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hexit {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 查看一组key是否存在
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Hmexit {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 返回值
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag = "3")]
        Integer(i64),
        #[prost(double, tag = "4")]
        Float(f64),
        #[prost(bool, tag = "5")]
        Bool(bool),
    }
}
#[derive(PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct Kvpair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
