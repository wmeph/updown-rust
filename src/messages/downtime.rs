use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};
use crate::command::{Parser, CliError};

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
    #[serde(skip)]
    pub(crate) token : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    page: Option<u32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<bool>,

}

impl DowntimeParams {
    pub(crate) fn parse(api_key: &str, matches: &ArgMatches<'_>) -> Result<DowntimeParams, CliError> {
        let mut params = DowntimeParamsBuilder::default();
        let mut parser = Parser::new(matches);

        params.api_key(api_key.to_string());
        params.token(parser.parse_value("token").unwrap());
        if let Some(page) = parser.parse_value("page") { params.page(page); }
        if let Some(results) = parser.parse_value("results") { params.results(results); }
        let params: DowntimeParams = params.build().unwrap();

        let validation_result : Result<(), ValidationErrors> = params.validate();
        match validation_result {
            Ok(()) => Ok(params),
            errors => {
                Err(CliError::BadArg("Unknown error".to_string()))
            }
        }
    }
}
