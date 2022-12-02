use std::collections::HashMap;

use log::{error, info};
use reqwest::{header, ClientBuilder, StatusCode};

use crate::model::cred::Cred;
use crate::model::sync_msg::SyncMsg;
use crate::CONFIG;

pub async fn sync_slaves(device_map: &HashMap<String, Cred>) {
    let url = format!("{}/api/data/post/synchronize_slaves", CONFIG.ionite_host);

    for (_type, cred) in device_map.iter() {
        match ClientBuilder::new()
            .build()
            .unwrap()
            .post(&url)
            .header("uuid", &cred.uuid)
            .header("password", &cred.pass)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&SyncMsg::new(&cred.devices))
            .send()
            .await
        {
            Ok(resp) => match resp.status() {
                StatusCode::CREATED => {
                    info!("Successful slave sync: {:?}", resp);
                }
                _ => error!("Failed while syncing: {:?}", resp),
            },
            Err(err) => error!("Failed to sync: {}", err),
        };
    }
}
