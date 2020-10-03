use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Validate, Deserialize, Debug)]

pub(crate) struct Downtime {
    error: String,
    started_at: String,
    ended_at: String,
    duration: u64,
}

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
    pub(crate) fn parse(api_key: &str, matches: &ArgMatches<'_>) -> DowntimeParams {
        let mut params = DowntimeParamsBuilder::default();
        params.api_key(api_key.to_string());
        if matches.is_present("page") {
            params.page(matches.value_of("page").unwrap().parse::<u32>().unwrap());
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
        params.build().unwrap()
    }
}
