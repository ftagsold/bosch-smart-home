use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    pub _type: String,
    pub certificate: String,
    #[serde(rename(serialize = "primaryRole", deserialize = "primaryRole"))]
    pub primary_role: String,
}
