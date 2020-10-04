use serde::{Deserialize, Serialize};

/// Config represents a minimal configuration for the updown client.
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Config {
    pub(crate) api_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) private_api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<String>,
}