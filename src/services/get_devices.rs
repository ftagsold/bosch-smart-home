use std::collections::HashMap;

use log::error;
use reqwest::{ClientBuilder, StatusCode};

use crate::model::cred::Cred;
use crate::model::device::Device;
use crate::model::device_map::DeviceMap;
use crate::services::sync_slaves::sync_slaves;
use crate::CONFIG;

pub async fn get_devices() -> HashMap<String, Cred> {
    let mut map = HashMap::new();
    let url = format!("https://{}:8444/smarthome/devices", CONFIG.controller_ip);

    match ClientBuilder::new()
        .identity(CONFIG.identity.clone())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            StatusCode::OK => {
                match resp.json::<Vec<Device>>().await {
                    Ok(devices) => {
                        map = DeviceMap::new(devices);
                        sync_slaves(&map).await;
                    }
                    Err(err) => error!("Invalid json while fetching devices: {:?}", err),
                };
            }
            _ => error!(
                "Controller returned error while fetching devices: {:?}",
                resp
            ),
        },
        Err(err) => error!("Failed to fetch devices: {}", err),
    };

    map
}
