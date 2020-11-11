use confy::ConfyError;
use serde::{Deserialize, Serialize};

/// Config represents a minimal configuration for the updown client.
#[derive(Serialize, Deserialize, Default, Builder)]
pub(crate) struct Config {
    pub(crate) api_key: String,

    #[builder(setter(skip))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) private_api_key: Option<String>,

    #[builder(setter(skip))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) user_agent: Option<String>,
}

impl Config {
    pub fn load_config() -> Result<Config, ConfyError> {
        confy::load("updown-rust")
    }
}
