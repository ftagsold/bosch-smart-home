extern crate tokio;

use std::env;
use std::fs::read_to_string;

use lazy_static::lazy_static;
use reqwest::{Error, Identity};

use crate::model::config::Config;
use crate::services::get_devices::get_devices;
use crate::services::poll_events::poll_events;
use crate::services::reg_new_client::reg_new_client;

mod model;
mod services;

lazy_static! {
    pub static ref CONFIG: Config = Config {
        identity: Identity::from_pkcs8_pem(
            read_to_string("./client-cert.pem").unwrap().as_bytes(),
            read_to_string("./client-key.pem").unwrap().as_bytes(),
        )
        .unwrap(),
        ionite_host: env::var("IONITE_HOST").unwrap_or("http://localhost:3012".to_string()),
        controller_ip: env::var("CONTROLLER_IP").unwrap_or("192.168.0.248".to_string())
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    if let Ok(password) = env::var("PASSWORD") {
        reg_new_client(password).await;
    }

    let device_map = get_devices().await;

    poll_events(&device_map).await;

    Ok(())
}
