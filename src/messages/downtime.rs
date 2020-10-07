use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::process::exit;
use validator::{Validate, ValidationError};
use std::error::Error;
use crate::messages::{MessageError};
use crate::cli::{Parser, CliError};

/// Downtime represents the output of /api/checks/:token/downtimes
#[derive(Serialize, Validate, Deserialize, Debug)]
pub(crate) struct Downtime {
    id: String,
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
    pub(crate) fn parse(api_key: &String, matches: &ArgMatches<'_>) -> Result<DowntimeParams, MessageError> {
        let mut params = DowntimeParamsBuilder::default();
        params.api_key(api_key.to_string());

        // TODO put this in parser intead.
        let mut successful_parse = true;
        let mut parser = Parser::new();
        let page: Result<Option<u32>, CliError> = parser.parse_value("page".to_string(), matches);
        if page.is_err() { successful_parse = false; } else {
            params.page(page.unwrap().unwrap());
        }

        if matches.is_present("results") {
            params.results(
                matches
                    .value_of("results")
                    .unwrap()
                    .parse::<bool>()
                    .unwrap(),
            );
        }
        if matches.is_present("group") {
            params.group(matches.value_of("group").unwrap().parse().unwrap());
        }

        let params: DowntimeParams = params.build().unwrap();
        params.validate();

        match successful_parse {
            true => Ok(params),
            false => Err(MessageError::ParseFailed(parser.parse_errors))
        }
    }
}
