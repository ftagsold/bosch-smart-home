use std::collections::HashMap;
use std::time::Duration;

use async_recursion::async_recursion;
use log::{error, info};
use reqwest::{header, ClientBuilder, StatusCode};

use crate::model::cred::Cred;
use crate::model::poll::{PollReq, PollResp, PollSub};
use crate::services::send_data::send_data;
use crate::CONFIG;

pub async fn poll_events(device_map: &HashMap<String, Cred>) {
    let url = format!("https://{}:8444/remote/json-rpc", CONFIG.controller_ip);

    match ClientBuilder::new()
        .identity(CONFIG.identity.clone())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(&url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&PollSub::new())
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            StatusCode::OK => {
                match resp.json::<Vec<PollResp>>().await {
                    Ok(subs) => {
                        if let Some(sub_id) = subs[0].clone().result {
                            rec_poll(&url, &sub_id, &device_map).await;
                        } else {
                            info!("Sub id err: {:?}", subs[0].error);
                        }
                    }
                    Err(err) => error!("Subscribe json err: {:?}", err),
                };
            }
            _ => error!("Controller subscribe err: {:?}", resp),
        },
        Err(err) => error!("Subscribe err: {}", err),
    };
}

#[async_recursion]
async fn rec_poll(url: &String, sub_id: &String, device_map: &HashMap<String, Cred>) {
    info!("Polling: {:?}", sub_id);

    match ClientBuilder::new()
        .identity(CONFIG.identity.clone())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .timeout(Duration::from_secs(30))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&PollReq::new(sub_id))
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            StatusCode::OK => {
                let text = resp.text().await.unwrap();

                info!("{:?}", text);
                send_data(text, device_map).await;
                rec_poll(url, sub_id, device_map).await;
            }
            _ => error!("Controller poll err: {:?}", resp),
        },
        Err(err) => {
            if err.is_timeout() {
                rec_poll(url, sub_id, device_map).await;
            } else {
                error!("Poll err: {}", err)
            }
        }
    };
}
