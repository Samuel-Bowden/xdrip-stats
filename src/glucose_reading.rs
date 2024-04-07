use crate::direction::Direction;
use crate::unit::Unit;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GlucoseReading {
    time: DateTime<Local>,
    value_mmol: f64,
    value_mgdl: f64,
    direction: Direction,
}

impl GlucoseReading {
    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn get_value(&self, unit: &Unit) -> f64 {
        match unit {
            Unit::Mmol => self.value_mmol,
            Unit::Mgdl => self.value_mgdl,
        }
    }

    pub fn get_time(&self) -> DateTime<Local> {
        self.time
    }
}
