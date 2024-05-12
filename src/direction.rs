use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub enum Direction {
    Flat,
    FortyFiveUp,
    FortyFiveDown,
    SingleUp,
    SingleDown,
    DoubleUp,
    DoubleDown,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Flat => "→",
            Self::FortyFiveUp => "↗",
            Self::FortyFiveDown => "↘",
            Self::SingleUp => "↑",
            Self::SingleDown => "↓",
            Self::DoubleUp => "⇈",
            Self::DoubleDown => "⇊",
        })
    }
}
