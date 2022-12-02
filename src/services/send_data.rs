use std::collections::HashMap;

use log::{error, info};
use reqwest::{header, ClientBuilder, StatusCode};

use crate::model::cred::Cred;
use crate::model::event::Event;
use crate::model::payload::{Data, Payload};
use crate::CONFIG;

pub async fn send_data(text: String, device_map: &HashMap<String, Cred>) {
    let url = format!("{}/api/data/post/device_data", CONFIG.ionite_host);

    if let Ok(events) = serde_json::from_str::<Vec<Event>>(text.as_str()) {
        for event in events.into_iter() {
            for result in event.result.into_iter() {
                'cred_loop: for (_, cred) in device_map.iter() {
                    for (_, device) in cred.devices.iter() {
                        if result.path.contains(device.id.as_str()) {
                            match ClientBuilder::new()
                                .build()
                                .unwrap()
                                .post(&url)
                                .header("uuid", &cred.uuid)
                                .header("slave", &device.id)
                                .header("password", &cred.pass)
                                .header(header::CONTENT_TYPE, "application/json")
                                .json(&Payload {
                                    payload: vec![Data { data: result }],
                                })
                                .send()
                                .await
                            {
                                Ok(resp) => match resp.status() {
                                    StatusCode::CREATED => {
                                        info!("Successfully pushed data");
                                    }
                                    _ => error!("Failed while pushing: {:?}", resp),
                                },
                                Err(err) => error!("Failed to push: {}", err),
                            };

                            break 'cred_loop;
                        }
                    }
                }
            }
        }
    }
}
