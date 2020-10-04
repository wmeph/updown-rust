use crate::print_errors;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::process::exit;
use validator::Validate;

#[derive(Serialize, Validate, Deserialize, Debug)]
pub(crate) struct Downtime {
    id: String,
    error: Option<String>,
    started_at: Option<String>,
    ended_at: Option<String>,
    duration: Option<u64>,
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
    pub(crate) fn parse(api_key: &String, matches: &ArgMatches<'_>) -> DowntimeParams {
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
        let params: DowntimeParams = params.build().unwrap();
            println!("{:#?}", serde_json::to_string(&params));

        let result = params.validate();
        match result {
            Ok(()) => {
                println!("valid!");
                return params;
            }
            Err(e) => {
                print_errors(e);
                exit(exitcode::DATAERR);
            }
        };
    }
}
