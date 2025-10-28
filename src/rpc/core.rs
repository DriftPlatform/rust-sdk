use std::io::Cursor;

use ciborium::Value;
use serde::{Deserialize, Serialize};

pub const DRIFTRPC_VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCRequest {
    pub drpc: String,
    pub method: String,
    pub params: Value,
    pub id: Value,
}

impl DriftRPCRequest {
    pub fn new(method: impl Into<String>, params: Value, id: impl Into<Value>) -> Self {
        Self {
            drpc: DRIFTRPC_VERSION.into(),
            method: method.into(),
            params,
            id: id.into(),
        }
    }

    pub fn maybe_from_cbor(vec: Vec<u8>) -> Option<Self> {
        ciborium::de::from_reader(Cursor::new(vec)).ok()
    }

    /// Create a new notification, e.g. a request with id == null
    pub fn notification(method: impl Into<String>, params: Value) -> Self {
        Self {
            drpc: DRIFTRPC_VERSION.into(),
            method: method.into(),
            params,
            id: Value::Null,
        }
    }

    /// Create a new call, e.g. a request with id == null and no parameters (params == null)
    pub fn call(method: impl Into<String>) -> Self {
        Self {
            drpc: DRIFTRPC_VERSION.into(),
            method: method.into(),
            params: Value::Null,
            id: Value::Null,
        }
    }
}

impl Into<Vec<u8>> for DriftRPCRequest {
    /// Marshal this DriftRPCRequest into a series of bytes (i.e., literally, serialise)
    fn into(self) -> Vec<u8> {
        let mut cbor = Vec::new();
        ciborium::ser::into_writer(&self, &mut cbor).expect("Failed to serialize DriftRPCRequest");
        cbor
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCResponse {
    pub drpc: String,
    pub result: Value,
    pub id: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCErrorDescription {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCError {
    pub drpc: String,
    pub error: Value,
    pub id: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DRPCMessage {
    DriftRPCRequest(DriftRPCRequest),
    DriftRPCResponse(DriftRPCResponse),
    DriftRPCError(DriftRPCError),
}

#[derive(Debug)]
pub enum DriftRPCErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
}

impl Into<i32> for DriftRPCErrorCode {
    /// Turn DriftRPCErrorCode into an integer code that can be included in a DriftRPC message.
    fn into(self) -> i32 {
        match self {
            DriftRPCErrorCode::ParseError => -32700,
            DriftRPCErrorCode::InvalidRequest => -32600,
            DriftRPCErrorCode::MethodNotFound => -32601,
            DriftRPCErrorCode::InvalidParams => -32602,
            DriftRPCErrorCode::InternalError => -32603,
        }
    }
}

impl Into<Value> for DriftRPCErrorCode {
    /// Turn DriftRPCErrorCode into an integer code that can be included in a DriftRPC message, wrapped as a Value
    fn into(self) -> Value {
        let c: i32 = self.into();
        c.into()
    }
}

impl DriftRPCError {
    pub fn new(code: DriftRPCErrorCode, message: String, id: Value) -> Self {
        Self {
            drpc: DRIFTRPC_VERSION.into(),
            error: Value::Map(vec![
                ("code".into(), code.into()),
                ("message".into(), message.into()),
            ]),
            id,
        }
    }
}
