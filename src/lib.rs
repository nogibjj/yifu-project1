use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    pub ip: String,
    pub version: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
}

#[derive(Debug)]
pub enum Service {
    IpApi,
    IpInfo,
}

pub struct Locator;

impl Locator {
    pub async fn get(ip: &str, service: Service) -> Result<Ip, Box<dyn Error>> {
        let url = match service {
            Service::IpApi => format!("https://ipapi.co/{ip}/json//"),
            Service::IpInfo => format!("https://ipapi.co/{ip}/json/"),
        };

        let response = reqwest::get(&url).await?.text().await?;
        let ip: Ip = serde_json::from_str(&response)?;

        Ok(ip)
    }
}
