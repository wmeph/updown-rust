use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::cli::{Parser, CliError};
use clap::ArgMatches;

/// Metrics represents the output of /api/checks/:token/metrics
/// Possible return values are an array of Metric messages or an error message.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Metrics {
    Error{error: Option<String>},
    Metrics(Vec<Metric>)
}


#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Metric {
    #[serde(skip_serializing_if = "Option::is_none")]
    apdex: Option<f32>,
    requests: Option<Requests>,
    timings: Option<Timings>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Requests {
    #[serde(skip_serializing_if = "Option::is_none")]
    samples: Option<u32>,
    failures: Option<u32>,
    satisfied: Option<u32>,
    tolerated: Option<u32>,
    by_response_time: Option<ResponseTimes>,
    timings: Option<Timings>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct ResponseTimes {
    under125: Option<u32>,
    under250: Option<u32>,
    under500: Option<u32>,
    under1000: Option<u32>,
    under2000: Option<u32>,
    under4000: Option<u32>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Timings {
    redirect: Option<u32>,
    namelookup: Option<u32>,
    connection: Option<u32>,
    handshake: Option<u32>,
    response: Option<u32>,
    total: Option<u32>,
}

#[derive(Clone, Validate, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(private, setter(strip_option))]
pub(crate) struct MetricParams {
    #[serde(rename = "api-key")]
    api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    from : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    to : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    group : Option<String>
}

impl MetricParams {
    pub(crate) fn parse(api_key: &String, matches: &ArgMatches<'_>) -> Result<MetricParams, CliError> {
        let mut params = MetricParamsBuilder::default();
        let mut parser = Parser::new();

        params.api_key(api_key.to_string());
        if let Some(from) = parser.parse_value("from", matches) { params.from(from); }
        if let Some(to) = parser.parse_value("to", matches) { params.to(to); }
        if let Some(group) = parser.parse_value("group", matches) { params.group(group); }
        let params: MetricParams = params.build().unwrap();

        params.validate();
        if parser.successful_parse().is_err() {
            Err(parser.successful_parse().unwrap_err())
        } else {
            Ok(params)
        }
    }
}
