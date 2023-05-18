use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IfconfigData {
    pub ip: String,
    //pub ip_decimal: u64,
    pub country: String,
    pub country_iso: String,
    pub country_eu: bool,
    pub region_name: String,
    pub region_code: String,
    pub zip_code: String,
    pub city: String,
    pub latitude: f32,
    pub longitude: f32,
    pub time_zone: String,
    pub asn: String,
    pub asn_org: String,
    pub user_agent: UserAgentData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAgentData {
    pub product: String,
    pub version: String,
    pub comment: Option<String>,
    pub raw_value: String,
}
