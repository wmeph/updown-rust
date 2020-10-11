use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::process::exit;
use validator::{Validate, ValidationError};
use std::error::Error;
use crate::messages::{MessageError};
use crate::cli::{Parser, CliError};
use std::string::ParseError;
use std::collections::HashMap;
use std::env::Args;
use std::str::FromStr;

/// Downtimes represents the output of /api/checks/:token/downtimes
/// Possible return values are an array of Downtime messages or an error message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Downtimes {
    Error{ error : Option<String>},
    Downtimes(Option<Vec<Downtime>>)
}

#[derive(Serialize, Validate, Deserialize, Debug)]
pub(crate) struct Downtime {
    id: Option<String>,
    error: Option<String>,
    started_at: Option<String>,
    ended_at: Option<String>,
    duration: Option<u64>,
}

/// DowntimeParams represents the parameters sent to /api/checks/:token/downtimes
#[derive(Clone, Validate, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(private, setter(strip_option))]
pub(crate) struct DowntimeParams {
    #[serde(rename = "api-key")]
    api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    page: Option<u32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<bool>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
}

impl DowntimeParams {
    pub(crate) fn parse(api_key: &str, matches: &ArgMatches<'_>) -> Result<DowntimeParams, CliError> {
        let mut params = DowntimeParamsBuilder::default();
        let mut parser = Parser::new();

        params.api_key(api_key.to_string());
        if let Some(page) = parser.parse_value("page", matches) { params.page(page); }
        if let Some(results) = parser.parse_value("results", matches) { params.results(results); }
        if let Some(group) = parser.parse_value("group", matches) { params.group(group); }
        let params: DowntimeParams = params.build().unwrap();

        params.validate();
        if parser.successful_parse().is_err() {
            Err(parser.successful_parse().unwrap_err())
        } else {
            Ok(params)
        }
    }
}
