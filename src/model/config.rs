use reqwest::Identity;

#[derive(Clone, Debug)]
pub struct Config {
    pub identity: Identity,
    pub ionite_host: String,
    pub controller_ip: String,
}
