use core::result::Result::Ok;

use std::collections::HashMap;

use crate::messages::check::{Check, CheckParams, Checks};
use crate::messages::downtime::{Downtime, DowntimeParams, Downtimes};
use crate::messages::MessageError;
use crate::config::Config;
use crate::{CHECKS_URL};
use reqwest::{Response, Url};
use std::process::exit;
use confy::ConfyError;
use crate::messages::metric::{MetricParams, Metrics};

/// Client is the API entry point.
/// A new Client instance will hold references to the user's full(?) and read-only API keys.
/// The read-only key is used internally for GET requests, the full key for POST and PUT requests.
/// The implementation defines methods that are used for the
pub(crate) struct Client {
    pub(crate) api_key: String,
    read_only_api_key: String,
    user_agent: String,
    http_client: reqwest::Client,
}

///
impl Client {

    pub(crate) async fn all(&self) -> Result<Checks, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned()).as_str()).unwrap();
        let resp = self
            .http_client
            .get(url)
            .query(&[("api-key", self.api_key.as_str())])
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub(crate) async fn check(&self, token: &str, metrics: bool) -> Result<Check, MessageError> {
        // TODO build the request parameters in a separate function
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api-key", self.api_key.as_str());
        if metrics {
            params.insert("metrics", "true");
        }
        let url =
            Url::parse_with_params((CHECKS_URL.to_owned() + "/" + token).as_str(), params).unwrap();
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub(crate) async fn downtimes(
        &self,
        token: &str,
        params: &DowntimeParams,
    ) -> Result<Downtimes, MessageError> {
        // -> Result<HashMap<String, Downtime>, MessageError>{
        let url =
            Url::parse((CHECKS_URL.to_owned() + "/" + token + "/downtimes").as_str()).unwrap();
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


    pub(crate) async fn metrics(
        &self,
        token: &str,
        params: &MetricParams,
    ) -> Result<Metrics, MessageError> {
        // -> Result<HashMap<String, Downtime>, MessageError>{
        let url =
            Url::parse((CHECKS_URL.to_owned() + "/" + token + "/downtimes").as_str()).unwrap();
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

    pub(crate) async fn add(
        &self,
        params: &CheckParams,
    ) -> Result<Check, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned()).as_str()).unwrap();
        let resp = self
            .http_client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub(crate) async fn update(
        &self,
        token: &str,
        params: &CheckParams,
    ) -> Result<Check, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned() + "/" + token).as_str()).unwrap();
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

    //TODO define message type, hashmap won't work.
    pub(crate) async fn delete(
        &self,
        token: &str,
    ) -> Result<HashMap<String, String>, MessageError> {
        let url = Url::parse((CHECKS_URL.to_owned() + "/" + token).as_str()).unwrap();
        let resp = self
            .http_client
            .delete(url)
            .query(&[("api-key", self.api_key.as_str())])
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub(crate) fn new(api_key : String, private_api_key : Option<String>, user_agent : Option<String>) -> Client {
        let mut client = Client{
            api_key : api_key,
            read_only_api_key: private_api_key.unwrap_or("".to_string()),
            user_agent: user_agent.unwrap_or("".to_string()),
            http_client: Default::default()
        };
        client
    }

    pub(crate) fn from_config() -> Result<Client, ConfyError> {
        let client = match Config::load_config() {
            Ok(config) => Client::new(config.api_key, config.private_api_key, config.user_agent),
            Err(e) => return Err(e)
        };
        Ok(client)
    }


}
