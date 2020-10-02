use core::result::Result::Ok;
use serde_json::value::Value;
use std::collections::HashMap;

use crate::checks::{Check, ChecksError, Metrics};
use crate::downtime::Downtime;
use crate::CHECKS_URL;
use reqwest::Url;
use serde::Serialize;
use std::cell::RefCell;
use tokio::macros::support::Future;

/// Client is the API entry point.
/// A new Client instance will hold references to the user's full(?) and read-only API keys.
/// The read-only key is used internally for GET requests, the full key for POST and PUT requests.
pub(crate) struct Client {
    api_key: String,
    read_only_api_key: String,
    user_agent: String,

    tokens: RefCell<HashMap<String, String>>,
    http_client: reqwest::Client,
}

///
impl Client {
    pub fn new(api_key: String, read_only_api_key: String, user_agent: String) -> Self {
        let checks_url = format!("{}{}{}", CHECKS_URL, "?api-key=", read_only_api_key);
        let mut tokens = RefCell::new(HashMap::new());
        let http_client = reqwest::Client::new();
        Client {
            api_key,
            read_only_api_key,
            user_agent,
            tokens,
            http_client,
        }
    }

    pub(crate) async fn all(&self) -> Result<Vec<Check>, ChecksError> {
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

    pub(crate) async fn check(&self, token: &str, metrics: bool) -> Result<Check, ChecksError> {
        // TODO build the request parameters in a separate function
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api-key", self.read_only_api_key.as_str());
        if metrics {
            params.insert("metrics", "true");
        }
        let url =
            Url::parse_with_params((CHECKS_URL.to_owned() + "/" + token).as_str(), params).unwrap();
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub(crate) async fn downtime(&self, token: &str, from : Option<&str>, to: Option<&str>, group: Option<&str>) -> Result<Metrics, ChecksError>{
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api-key", self.read_only_api_key.as_str());
        if from.is_some() {
            params.insert("from", from.unwrap());
        }
        if to.is_some() {
            params.insert("to", to.unwrap());
        }
        if group.is_some() {
            params.insert("group", group.unwrap());
        }
        let url =
            Url::parse_with_params((CHECKS_URL.to_owned() + "/" + token).as_str(), params).unwrap();
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    pub(crate) async fn add(&self, url: &str, params: &Check) -> Result<Check, ChecksError> {
        let url = Url::parse((CHECKS_URL.to_owned()).as_str()).unwrap();
        let resp = self
            .http_client
            .post(url)
            .query(&[("api-key", self.api_key.as_str())])
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    pub(crate) async fn update(&self, token: &str, params: &Check) -> Result<Check, ChecksError> {
        let url = Url::parse((CHECKS_URL.to_owned() + "/" + token).as_str()).unwrap();
        let resp = self
            .http_client
            .put(url)
            .query(&[("api-key", self.api_key.as_str())])
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    //TODO define message type, hashmap won't work.
    pub(crate) async fn delete(&self, token: &str) -> Result<HashMap<String, String>, ChecksError>{

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
}
