#[derive(Debug)]
pub enum XdripError {
    RequestFailed,
    GlucoseReadingOld
}

impl std::fmt::Display for XdripError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequestFailed => write!(f, "Failed to send request to database"),
            Self::GlucoseReadingOld => write!(f, "No glucose reading for past 5 minutes"),
        }
    }
}

impl std::error::Error for XdripError {}
