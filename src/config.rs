use confy::ConfyError;
use serde::{Deserialize, Serialize};

/// Config represents a minimal configuration for the updown client.
#[derive(Serialize, Deserialize, Default, Builder)]
pub(crate) struct Config {
    pub(crate) api_key: String,

    pub(crate) private_api_key: String,

    pub(crate) user_agent: String,
}

impl Config {
    pub fn load_config() -> Result<Config, ConfyError> {
        confy::load("updown-rust")
    }
}
