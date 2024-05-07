use crate::direction::Direction;
use crate::status::Status;
use crate::unit::Unit;
use std::fmt::Display;

pub struct Text<'a> {
    value: f64,
    unit: &'a Unit,
    direction: Direction,
    status: &'a Status,
}

impl<'a> Text<'a> {
    pub fn new(value: f64, unit: &'a Unit, direction: Direction, status: &'a Status) -> Self {
        Self {
            value,
            unit,
            direction,
            status,
        }
    }
}

impl<'a> Display for Text<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1} {} {} {}",
            self.value, self.unit, self.direction, self.status
        )
    }
}
