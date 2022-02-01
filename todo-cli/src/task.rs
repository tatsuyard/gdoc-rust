use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
}