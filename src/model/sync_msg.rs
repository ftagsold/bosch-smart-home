use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::model::device::Device;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SyncMsg {
    slaves: Vec<Slave>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Slave {
    pub name: String,
    pub slave: String,
}

impl SyncMsg {
    pub fn new(devices: &HashMap<String, Device>) -> Self {
        Self {
            slaves: devices
                .into_iter()
                .map(|(id, device)| Slave {
                    slave: id.to_owned(),
                    name: device.to_owned().name,
                })
                .collect(),
        }
    }
}
