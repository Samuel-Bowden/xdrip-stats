use self::output::Output;
use crate::glucose_reading::GlucoseReading;
use crate::status::Status;
use crate::{XdripState, XdripStats};
use anyhow::Result;

mod output;
mod text;
mod tooltip;

pub struct WaybarStats(XdripState);

impl WaybarStats {
    pub fn new(state: XdripState) -> Self {
        Self(state)
    }
}

impl XdripStats for WaybarStats {
    fn output_reading(&mut self, reading: &GlucoseReading) -> Result<()> {
        Output::new(reading, &Status::Ok, &self.0.unit).print_json()
    }

    fn output_error(&self, msg: String) -> Result<()> {
        match &self.0.last_reading {
            Some(reading) => Output::new(reading, &Status::Error(msg), &self.0.unit),
            None => Output::error_with_no_data(msg),
        }
        .print_json()
    }

    fn state_mut(&mut self) -> &mut XdripState {
        &mut self.0
    }
}
