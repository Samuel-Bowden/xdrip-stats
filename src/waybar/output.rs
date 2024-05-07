use super::text::Text;
use super::tooltip::Tooltip;
use crate::glucose_reading::GlucoseReading;
use crate::status::Status;
use crate::status::ERROR_ICON;
use crate::unit::Unit;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    text: String,
    tooltip: String,
}

impl Output {
    pub fn new(reading: &GlucoseReading, status: &Status, unit: &Unit) -> Self {
        Self {
            text: Text::new(
                reading.get_value(unit),
                unit,
                reading.get_direction(),
                status,
            )
            .to_string(),
            tooltip: Tooltip::new(reading.get_time(), status).to_string(),
        }
    }

    pub fn error_with_no_data(msg: String) -> Self {
        Self {
            text: format!("Error {ERROR_ICON}"),
            tooltip: format!("Error: {}", msg),
        }
    }

    pub fn print_json(&self) -> Result<()> {
        println!("{}", serde_json::to_string(&self)?);
        Ok(())
    }
}
