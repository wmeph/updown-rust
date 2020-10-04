use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::messages::metric::Metrics;
use crate::print_errors;
use clap::ArgMatches;
use std::process::exit;
use validator::{Validate, ValidationError};

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
    metrics: Option<Metrics>,
}

fn validate_period(period: u32) -> Result<(), ValidationError> {
    match period {
        // 15, 30, 60, 120, 300, 600, 1800 or 3600
        15 | 30 | 60 | 120 | 300 | 600 | 1800 | 3600 => Ok(()),
        _ => Err(ValidationError::new(concat!("Invalid value for period"))),
    }
}


#[derive(Clone, Validate, Serialize, Deserialize, Debug, Default, Builder)]
#[builder(private, setter(strip_option))]
pub struct CheckParams {
    #[serde(rename = "api-key")]
    api_key: String,
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
    pub(crate) fn parse_update(api_key: &String, matches: &ArgMatches<'_>) -> CheckParams {
        let mut params = CheckParamsBuilder::default();
        params.api_key(api_key.to_string());
        parse(params, matches)
    }

    pub(crate) fn parse_add(api_key: &String, url: String, matches: &ArgMatches<'_>) -> CheckParams {
        let mut params = CheckParamsBuilder::default();
        params.api_key(api_key.to_string());
        params.url(url);
        parse(params, matches)
    }
}

fn parse(mut params: CheckParamsBuilder, matches: &ArgMatches<'_>) -> CheckParams {
    if matches.is_present("url") {
        params.url(matches.value_of("url").unwrap().parse().unwrap());
    }
    if matches.is_present("period") {
        let period = matches.value_of("period").unwrap().parse::<u32>().unwrap();
        params.period(period);
    }
    if matches.is_present("apdex_t") {
        let apdex_t = matches.value_of("apdex_t").unwrap().parse::<f32>().unwrap();
        params.apdex_t(apdex_t);
    }
    if matches.is_present("enabled") {
        params.enabled(
            matches
                .value_of("enabled")
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        );
    }
    if matches.is_present("published") {
        params.published(
            matches
                .value_of("published")
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        );
    }
    if matches.is_present("alias") {
        params.alias(matches.value_of("alias").unwrap().parse().unwrap());
    }
    if matches.is_present("string_match") {
        params.string_match(matches.value_of("string_match").unwrap().parse().unwrap());
    }
    if matches.is_present("mute_until") {
        params.mute_until(matches.value_of("mute_until").unwrap().parse().unwrap());
    }
    if matches.is_present("http_verb") {
        params.http_verb(matches.value_of("http_verb").unwrap().parse().unwrap());
    }
    if matches.is_present("http_body") {
        params.http_body(matches.value_of("http_body").unwrap().parse().unwrap());
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
    let result = params.validate();
    match result {
        Ok(()) => {
            return params;
        }
        Err(e) => {
            print_errors(e);
            exit(exitcode::DATAERR);
        }
    };
}
// let mut check = Check::new();
//     if matches.is_present("url") {
//         check.url(matches.value_of("url").unwrap().parse().unwrap());
//     }
//     if matches.is_present("period") {
//         let period = matches.value_of("period").unwrap().parse::<u32>().unwrap();
//         check.period(period);
//     }
//     if matches.is_present("apdex_t") {
//         let apdex_t = matches.value_of("apdex_t").unwrap().parse::<f32>().unwrap();
//         check.apdex_t(apdex_t);
//     }
//     if matches.is_present("enabled") {
//         check.enabled(
//             matches
//                 .value_of("enabled")
//                 .unwrap()
//                 .parse::<bool>()
//                 .unwrap(),
//         );
//     }
//     if matches.is_present("published") {
//         check.published(
//             matches
//                 .value_of("published")
//                 .unwrap()
//                 .parse::<bool>()
//                 .unwrap(),
//         );
//     }
//     if matches.is_present("alias") {
//         check.alias(matches.value_of("alias").unwrap().parse().unwrap());
//     }
//     if matches.is_present("string_match") {
//         check.string_match(matches.value_of("string_match").unwrap().parse().unwrap());
//     }
//     if matches.is_present("mute_until") {
//         check.mute_until(matches.value_of("mute_until").unwrap().parse().unwrap());
//     }
//     if matches.is_present("http_verb") {
//         check.http_verb(matches.value_of("http_verb").unwrap().parse().unwrap());
//     }
//     if matches.is_present("http_body") {
//         check.http_body(matches.value_of("http_body").unwrap().parse().unwrap());
//     }
//     if matches.is_present("disabled_locations") {
//         unimplemented!()
//         //check.disabled_locations(matches.value_of("disabled_locations").unwrap().parse().unwrap());
//     }
//     if matches.is_present("custom_headers") {
//         unimplemented!()
//         // check.custom_headers(matches.value_of("custom_headers").unwrap().parse().unwrap());
//     }
//
//     let result = check.validate();
//     match result {
//         Ok(()) => {
//             println!("valid!");
//             return check;
//         }
//         Err(e) => {
//             print_errors(e);
//             exit(exitcode::DATAERR);
//         }
//     };

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
