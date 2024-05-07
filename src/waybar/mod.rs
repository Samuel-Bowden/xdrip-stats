use self::output::Output;
use crate::glucose_reading::GlucoseReading;
use crate::status::Status;
use crate::unit::Unit;
use crate::XdripStats;
use anyhow::Result;

mod output;
mod text;
mod tooltip;

pub struct WaybarStats {
    pub unit: Unit,
    last_reading: Option<GlucoseReading>,
}

impl WaybarStats {
    pub fn new(unit: Unit) -> Self {
        Self {
            unit,
            last_reading: None,
        }
    }
}

impl XdripStats for WaybarStats {
    fn output_reading(&mut self, reading: GlucoseReading) -> Result<()> {
        Output::new(&reading, &Status::Ok, &self.unit).print_json()?;
        self.last_reading = Some(reading);
        Ok(())
    }

    fn output_error(&self, msg: String) -> Result<()> {
        match &self.last_reading {
            Some(reading) => Output::new(reading, &Status::Error(msg), &self.unit),
            None => Output::error_with_no_data(msg),
        }
        .print_json()
    }
}
