use std::io::Cursor;

use serde::{Deserialize, Serialize};

pub const DRIFTRPC_VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCRequest {
    pub drpc: String,
    pub method: String,
    pub params: ciborium::Value,
    pub id: ciborium::Value,
}

impl DriftRPCRequest {
    pub fn new(
        method: impl Into<String>,
        params: ciborium::Value,
        id: impl Into<ciborium::Value>,
    ) -> Self {
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCResponse {
    pub drpc: String,
    pub result: ciborium::Value,
    pub id: ciborium::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCErrorDescription {
    pub code: i64,
    pub message: String,
    pub data: Option<ciborium::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DriftRPCError {
    pub drpc: String,
    pub error: ciborium::Value,
    pub id: ciborium::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DRPCResponseOrError {
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

impl DriftRPCError {
    pub fn new(code: DriftRPCErrorCode, message: String, id: ciborium::Value) -> Self {
        let code_value = match code {
            DriftRPCErrorCode::ParseError => -32700,
            DriftRPCErrorCode::InvalidRequest => -32600,
            DriftRPCErrorCode::MethodNotFound => -32601,
            DriftRPCErrorCode::InvalidParams => -32602,
            DriftRPCErrorCode::InternalError => -32603,
        };
        Self {
            drpc: DRIFTRPC_VERSION.into(),
            error: ciborium::Value::Map(vec![
                ("code".into(), code_value.into()),
                ("message".into(), message.into()),
            ]),
            id,
        }
    }
}
