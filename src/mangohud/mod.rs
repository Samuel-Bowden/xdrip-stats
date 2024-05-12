use crate::glucose_reading::GlucoseReading;
use crate::state::XdripState;
use crate::XdripStats;
use anyhow::Result;

pub struct MangoHudStats(XdripState);

impl MangoHudStats {
    pub fn new(state: XdripState) -> Self {
        Self(state)
    }
}

impl XdripStats for MangoHudStats {
    fn output_reading(&mut self, _reading: &GlucoseReading) -> Result<()> {
        todo!("MangoHud output not supported yet");
    }

    fn output_error(&self, _msg: String) -> Result<()> {
        todo!()
    }

    fn state_mut(&mut self) -> &mut XdripState {
        &mut self.0
    }
}
