use core::result::Result::Ok;
use serde_json::value::Value;
use std::collections::HashMap;

use crate::checks::{Check, ChecksError};
use crate::downtime::Downtime;
use crate::CHECKS_URL;
use std::cell::RefCell;
use tokio::macros::support::Future;
use reqwest::Url;

/// Client is the API entry point.
/// A new Client instance will hold references to the user's full(?) and read-only API keys.
/// The read-only key is used internally for GET requests, the  key for POST and PUT requests.
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

    pub(crate) async fn checks(&self) -> Result<Vec<Check>, ChecksError> {
        Ok(self.get(String::new(), String::new(), None).await?)
    }

    pub(crate) async fn summary(&self, token: &str, metrics: bool) -> Result<Check, ChecksError> {
        // TODO build the request parameters in a separate function
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("api-key", self.read_only_api_key.as_str());
        if metrics {
            params.insert("metrics", "true");
        }
        let url = Url::parse_with_params((CHECKS_URL.to_owned()+ "/" + token).as_str(), params).unwrap();
        let resp = reqwest::get(url).await?.json().await?;
        Ok(resp)
    }

    fn request() {

    }


    pub(crate) async fn downtimes(
        &mut self,
        token: String,
        params: Option<String>,
    ) -> Result<Vec<Downtime>, ChecksError> {
        Ok(self.get("/downtimes".to_string(), token, params).await?)
    }

    pub(crate) async fn metrics(&mut self, token: String) -> Result<Value, ChecksError> {
        Ok(self.get("/metrics".to_string(), token, None).await?)
    }

    pub(crate) async fn post_check(
        &mut self,
        token: String,
        check: Check,
    ) -> Result<Check, ChecksError> {
        Ok(self.post(String::new(), token, check).await?)
    }

    pub(crate) async fn update_check(
        &mut self,
        token: String,
        check: Check,
    ) -> Result<Check, ChecksError> {
        Ok(self.put(String::new(), token, check).await?)
    }

    async fn get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        segment: String,
        token: String,
        params: Option<String>,
    ) -> Result<T, ChecksError> {
        let resp = reqwest::get(
            self.get_url_for_token(segment, token, self.read_only_api_key.to_string(), params)
                .as_str(),
        )
        .await?
        .json()
        .await?;
        Ok(resp)
    }

    // Creates a POST request with a Check parameter
    //
    // # Arguments
    //
    // * `token` - The API token (may be blank)
    // * `api_key` - The API key, which will be the standard or read-only key from the client
    // * check - the Check data to update
    async fn post<T: for<'de> serde::Deserialize<'de>>(
        &self,
        segment: String,
        token: String,
        check: Check,
    ) -> Result<T, ChecksError> {
        let resp = self
            .http_client
            .post(
                self.get_url_for_token(segment, token, self.api_key.to_string(), None)
                    .as_str(),
            )
            .json(&check)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    // Creates a PUT request with a Check parameter
    //
    // # Arguments
    //
    // * `token` - The API token (may be blank)
    // * `api_key` - The API key, which will be the standard or read-only key from the client
    // * check - the Check data to update
    async fn put<T: for<'de> serde::Deserialize<'de>>(
        &self,
        segment: String,
        token: String,
        check: Check,
    ) -> Result<T, ChecksError> {
        let resp = self
            .http_client
            .put(
                self.get_url_for_token(segment, token, self.api_key.to_string(), None)
                    .as_str(),
            )
            .json(&check)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    async fn delete<T: for<'de> serde::Deserialize<'de>>(
        &self,
        segment: String,
        token: String,
    ) -> Result<T, ChecksError> {
        let resp = self
            .http_client
            .delete(
                self.get_url_for_token(segment, token, self.api_key.to_string(), None)
                    .as_str(),
            )
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    // Builds the URL for a request
    //
    // # Arguments
    //
    // * `url_segment` - The URL segment following "http://updown.io/api/checks" (may be blank)
    // * `token` - The API token (may be blank)
    // * `api_key` - The API key, which will be the standard or read-only key from the client
    fn get_url_for_token(
        &self,
        url_segment: String,
        token: String,
        api_key: String,
        params: Option<String>,
    ) -> String {
        let key = format!("{}{}{}", token, url_segment, api_key);
        let mut token_string = String::new();
        if token.len() > 0 {
            token_string = "/".to_string() + token.as_str();
        }
        self.tokens
            .borrow_mut()
            .entry(key.clone())
            .or_insert(format!(
                "{}{}{}{}{}{}",
                CHECKS_URL,
                token_string,
                url_segment,
                "?api-key=",
                api_key,
                params.unwrap_or("".to_string())
            ));
        let url = (&self.tokens.borrow()[&key]).parse().unwrap();
        println!("{:#?}", url);
        url
    }

}
