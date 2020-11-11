use core::result::Result::Ok;

use std::collections::HashMap;

use crate::messages::check::{Check, CheckParams, Checks};
use crate::messages::downtime::{DowntimeParams, Downtimes};
use crate::messages::metric::{Metrics, MetricsParams};
use crate::messages::MessageError;
use crate::CHECKS_URL;
use reqwest::Url;

/// Client is the API entry point.
/// A new Client instance will hold references to the user's full(?) and read-only API keys.
/// The read-only key is used internally for GET requests, the full key for POST and PUT requests.
/// The implementation defines methods that are used for the
pub(crate) struct Client<'a> {
    pub(crate) api_key: &'a str,
    read_only_api_key: String,
    user_agent: String,
    http_client: reqwest::Client,
}

///
impl Client<'_> {

    pub async fn all(&self) -> Result<Checks, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned()).as_str()).unwrap();
        let resp = self
            .http_client
            .get(url)
            .query(&[("api-key", self.api_key)])
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn check(&self, token: &str, metrics: bool) -> Result<Check, MessageError> {
        // TODO build the request parameters in a separate function
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api-key", self.api_key);
        if metrics {
            params.insert("metrics", "true");
        }
        let url =
            Url::parse_with_params((CHECKS_URL.to_owned() + "/" + token).as_str(), params).unwrap();
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub async fn downtimes(&self, params: &DowntimeParams<'_>) -> Result<Downtimes, MessageError> {
        // -> Result<HashMap<String, Downtime>, MessageError>{
        let url = Url::parse((CHECKS_URL.to_owned() + "/" + params.token + "/downtimes").as_str())
            .unwrap();
        let resp = self
            .http_client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn metrics(&self, params: &MetricsParams<'_>) -> Result<Metrics, MessageError> {
        // -> Result<HashMap<String, Downtime>, MessageError>{
        let url =
            Url::parse((CHECKS_URL.to_owned() + "/" + params.token + "/metrics").as_str()).unwrap();
        let resp = self
            .http_client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn update(&self, params: &CheckParams) -> Result<Check, MessageError> {
        let url =
            Url::parse((CHECKS_URL.to_owned() + "/" + &params.token.as_str()).as_str()).unwrap();
        let resp = self
            .http_client
            .put(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub async fn delete(&self, token: &str) -> Result<HashMap<String, String>, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned() + "/" + token).as_str()).unwrap();
        let resp = self
            .http_client
            .delete(url)
            .query(&[("api-key", self.api_key)])
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub fn new(
        api_key: &str,
        private_api_key: Option<String>,
        user_agent: Option<String>,
    ) -> Client {
        let client = Client {
            api_key,
            read_only_api_key: private_api_key.unwrap_or(String::from("")),
            user_agent: user_agent.unwrap_or("".to_string()),
            http_client: Default::default(),
        };
        client
    }
}
