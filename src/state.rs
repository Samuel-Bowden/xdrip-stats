use crate::{glucose_reading::GlucoseReading, unit::Unit};
use influxdb::Client;

pub struct XdripState {
    pub unit: Unit,
    pub last_reading: Option<GlucoseReading>,
    pub client: Client,
}

impl XdripState {
    pub fn new(unit: Unit, url: String, database: String, token: String) -> Self {
        Self {
            unit,
            last_reading: None,
            client: Client::new(url, database).with_token(token),
        }
    }
}
