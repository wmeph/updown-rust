use serde::{Deserialize, Serialize};
use confy::ConfyError;

/// Config represents a minimal configuration for the updown client.
#[derive(Serialize, Deserialize, Default)]
#[derive(Builder)]
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
    pub fn load_config() -> Result<Config, ConfyError>{
        confy::load("updown-rust")
    }
}

// impl Config {
//     pub fn new(api_key: String) -> Config {
//         Config {
//             api_key,
//             private_api_key: None,
//             user_agent: None
//         }
//     }
//
//     pub fn private_api_key<'a>(&'a mut self, key : String) -> &'a mut Config {
//         self.private_api_key = Option::from(key);
//         self
//     }
//
//     pub fn user_agent<'a>(&'a mut self, user_agent: String) -> &'a mut Config {
//         self.user_agent = Option::from(user_agent);
//         self
//     }
// }