use std::fs::read_to_string;

use base64::encode;
use log::{error, info};
use reqwest::{header, ClientBuilder, StatusCode};

use crate::model::client::Client;
use crate::CONFIG;

pub async fn reg_new_client(password: String) {
    let url = format!("https://{}:8443/smarthome/clients", CONFIG.controller_ip);

    let c = Client {
        _type: "client".to_string(),
        id: "oss_ionite".to_string(),
        name: "OSS Ionite".to_string(),
        primary_role: "ROLE_RESTRICTED_CLIENT".to_string(),
        certificate: read_to_string("./client-cert.pem").unwrap(),
    };

    match ClientBuilder::new()
        .identity(CONFIG.identity.clone())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Systempassword", encode(password))
        .json(&c)
        .send()
        .await
    {
        Ok(resp) => match resp.status() {
            StatusCode::CREATED => info!("Successfully created client"),
            _ => error!("Failed to create new client: {:?}", resp),
        },
        Err(err) => error!("Error while creating client: {}", err),
    };
}
