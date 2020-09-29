use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Validate, Deserialize, Debug)]
pub(crate) struct Downtime {
    error: String,
    started_at: String,
    ended_at: String,
    duration: u64,
}
