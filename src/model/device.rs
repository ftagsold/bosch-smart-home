use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum Status {
    AVAILABLE,
    #[default]
    UNAVAILABLE,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    pub _type: String,
    pub status: Status,
    pub serial: Option<String>,
    pub profile: Option<String>,
    #[serde(rename(serialize = "roomId", deserialize = "roomId"))]
    pub room_id: Option<String>,
    pub manufacturer: String,
    #[serde(rename(serialize = "deviceModel", deserialize = "deviceModel"))]
    pub device_model: String,
    #[serde(rename(serialize = "rootDeviceId", deserialize = "rootDeviceId"))]
    pub root_device_id: String,
    #[serde(rename(serialize = "deviceServiceIds", deserialize = "deviceServiceIds"))]
    pub device_service_ids: Vec<String>,
}
