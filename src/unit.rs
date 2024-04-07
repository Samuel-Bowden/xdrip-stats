use clap::ValueEnum;
use std::fmt::Display;

#[derive(ValueEnum, Clone)]
pub enum Unit {
    Mmol,
    Mgdl,
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Unit::Mmol => "mmol/L",
            Unit::Mgdl => "mg/dl",
        })
    }
}
