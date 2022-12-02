use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PollSub {
    pub jsonrpc: String,
    pub method: String,
    pub params: (String, Option<String>),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PollReq {
    pub jsonrpc: String,
    pub method: String,
    pub params: (String, i32),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PollUnsub {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PollResp {
    pub jsonrpc: String,
    pub result: Option<String>,
    pub error: Option<PollError>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PollError {
    pub code: u32,
    pub message: String,
}

impl PollSub {
    pub fn new() -> Vec<Self> {
        vec![Self {
            jsonrpc: "2.0".to_string(),
            method: "RE/subscribe".to_string(),
            params: ("com/bosch/sh/remote/*".to_string(), None),
        }]
    }
}

impl PollReq {
    pub fn new(id: &String) -> Vec<Self> {
        vec![Self {
            jsonrpc: "2.0".to_string(),
            method: "RE/longPoll".to_string(),
            params: (id.to_string(), 30),
        }]
    }
}
