use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::command::{CliError, Parser};
use crate::messages::metric::Message;
use clap::ArgMatches;
use validator::{Validate, ValidationError};

/// Checks represents the output of /api/checks/:token/Checks
/// Possible return values are an array of Check messages or an error message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Checks {
    Error { error: Option<String> },
    Checks(Vec<Check>),
}

/// Check represents the output of /api/checks/:token; Vec<Check> is deserialized from /api/checks
#[derive(Clone, Validate, Serialize, Deserialize, Debug, Default)]
pub struct Check {
    pub(crate) token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    // Not in POST or PUT params
    #[serde(skip_serializing)]
    last_status: Option<u16>,
    #[serde(skip_serializing)]
    uptime: Option<f32>,
    #[serde(skip_serializing)]
    down: Option<bool>,
    #[serde(skip_serializing)]
    down_since: Option<String>,
    #[serde(skip_serializing)]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_period")]
    period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apdex_t: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled_locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_check_at: Option<String>,
    #[serde(skip_serializing)]
    next_check_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mute_until: Option<String>, //?

    #[validate(url)]
    #[serde(skip_serializing)]
    favicon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_verb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_body: Option<String>,

    #[serde(skip_serializing)]
    ssl: Option<Ssl>,

    #[serde(skip_serializing)]
    metrics: Option<Message>,
}

fn validate_period(period: u32) -> Result<(), ValidationError> {
    match period {
        // 15, 30, 60, 120, 300, 600, 1800 or 3600
        15 | 30 | 60 | 120 | 300 | 600 | 1800 | 3600 => Ok(()),
        _ => Err(ValidationError::new(concat!("Invalid value for period"))),
    }
}

/// CheckParams represents the parameters sent to PUT /api/checks:token and POST /api/checks
#[derive(Clone, Validate, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(setter(strip_option))]
pub struct CheckParams {
    #[serde(rename = "api-key")]
    api_key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    #[builder(default = "None")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(custom = "validate_period")]
    #[builder(default = "None")]
    period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    apdex_t: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    string_match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    last_check_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    mute_until: Option<String>,
    //?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    http_verb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    http_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    disabled_locations: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    custom_headers: Option<HashMap<String, String>>,
}

impl CheckParams {
    /// Parses parameters for the update request: PUT /api/checks/:token
    pub(crate) fn parse_update(
        api_key: &str,
        matches: &ArgMatches<'_>,
    ) -> Result<CheckParams, CliError> {
        let mut params = CheckParamsBuilder::default();
        params.api_key(api_key.to_string());
        CheckParams::parse(params, matches)
    }

    pub(crate) fn parse(
        mut params: CheckParamsBuilder,
        matches: &ArgMatches<'_>,
    ) -> Result<CheckParams, CliError> {
        let mut parser = Parser::new(matches);
        let token: String = parser.parse_value("token").unwrap();
        if !token.is_empty() {
            params.token(token);
        }
        if let Some(url) = parser.parse_value("url") {
            params.url(url);
        }
        if let Some(period) = parser.parse_value("period") {
            params.period(period);
        }
        if let Some(apdex_t) = parser.parse_value("apdex_t") {
            params.apdex_t(apdex_t);
        }
        if let Some(enabled) = parser.parse_value("enabled") {
            params.enabled(enabled);
        }
        if let Some(published) = parser.parse_value("published") {
            params.published(published);
        }
        if let Some(string_match) = parser.parse_value("string_match") {
            params.string_match(string_match);
        }
        if let Some(alias) = parser.parse_value("alias") {
            params.alias(alias);
        }
        if let Some(mute_until) = parser.parse_value("mute_until") {
            params.mute_until(mute_until);
        }
        if let Some(http_verb) = parser.parse_value("http_verb") {
            params.http_verb(http_verb);
        }
        if let Some(http_body) = parser.parse_value("http_body") {
            params.http_body(http_body);
        }
        if matches.is_present("disabled_locations") {
            unimplemented!()
            //params.disabled_locations(matches.value_of("disabled_locations").unwrap().parse().unwrap());
        }
        if matches.is_present("custom_headers") {
            unimplemented!()
            // params.custom_headers(matches.value_of("custom_headers").unwrap().parse().unwrap());
        }
        let params: CheckParams = params.build().unwrap();

        match params.validate() {
            Ok(p) => Ok(params),
            Err(e) => Err(CliError::BadArg("Invalid parameters".to_string()))
        }
    }
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Ssl {
    #[serde(skip_serializing_if = "Option::is_none")]
    tested_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}
