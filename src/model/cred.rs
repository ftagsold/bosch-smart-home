use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::model::device::Device;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Cred {
    pub uuid: String,
    pub pass: String,
    pub _type: String,
    #[serde(default)]
    pub devices: HashMap<String, Device>,
}
