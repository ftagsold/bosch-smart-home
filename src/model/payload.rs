use serde::Deserialize;
use serde::Serialize;

use crate::model::event::Result;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Data {
    pub data: Result,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Payload {
    pub payload: Vec<Data>,
}
