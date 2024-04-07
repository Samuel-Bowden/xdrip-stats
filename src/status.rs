use std::fmt::Display;

pub const OK_ICON: &str = "✓";
pub const ERROR_ICON: &str = "⚠";

pub enum Status {
    Ok,
    Error(&'static str),
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ok => OK_ICON,
            Self::Error(_) => ERROR_ICON,
        })
    }
}
