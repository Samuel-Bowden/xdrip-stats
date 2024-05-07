use crate::glucose_reading::GlucoseReading;
use crate::unit::Unit;
use crate::XdripStats;
use anyhow::Result;

pub struct MangoHudStats {
    pub unit: Unit,
    _last_reading: Option<GlucoseReading>,
}

impl MangoHudStats {
    pub fn new(unit: Unit) -> Self {
        Self {
            unit,
            _last_reading: None,
        }
    }
}

impl XdripStats for MangoHudStats {
    fn output_reading(&mut self, _reading: GlucoseReading) -> Result<()> {
        todo!("MangoHud output not supported yet");
    }

    fn output_error(&self, _msg: String) -> Result<()> {
        todo!()
    }
}
