use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Event {
    pub jsonrpc: String,
    pub result: Vec<Result>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Result {
    pub id: String,
    pub path: String,
    pub state: Value,
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    pub _type: String,
    #[serde(rename(serialize = "deviceId", deserialize = "deviceId"))]
    pub device_id: String,
}
