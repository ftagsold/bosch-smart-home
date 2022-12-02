use std::collections::HashMap;
use std::fs::read_to_string;

use serde::Deserialize;
use serde::Serialize;

use crate::model::cred::Cred;
use crate::model::device::Device;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeviceMap;

impl DeviceMap {
    pub fn new(devices: Vec<Device>) -> HashMap<String, Cred> {
        let mut map = HashMap::from_iter(
            serde_json::from_str::<Vec<Cred>>(
                read_to_string("./devices.json")
                    .expect("Devices.json not present")
                    .as_str(),
            )
            .expect("Serde devices.json parse failed")
            .into_iter()
            .map(|cred| (cred._type.clone(), cred)),
        );

        for device in devices.into_iter() {
            if let Some(cred) = map.get_mut(&device.device_model) {
                cred.devices.insert(device.id.clone(), device);
            }
        }

        map
    }
}
