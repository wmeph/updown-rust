use core::option::Option::{None, Some};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Default, Builder)]
#[builder(default, setter(into, strip_option))]
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

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Builder)]
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

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Builder)]
pub struct Metrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    apdex: Option<f32>,
    requests: Option<Requests>,
    timings: Option<Timings>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Builder)]
pub struct Requests {
    #[serde(skip_serializing_if = "Option::is_none")]
    samples: Option<u32>,
    failures: Option<u32>,
    satisfied: Option<u32>,
    tolerated: Option<u32>,
    by_response_time: Option<ResponseTimes>,
    timings: Option<Timings>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Builder)]
pub struct ResponseTimes {
    under125: Option<u32>,
    under250: Option<u32>,
    under500: Option<u32>,
    under1000: Option<u32>,
    under2000: Option<u32>,
    under4000: Option<u32>,
}

#[derive(Clone, Serialize, Validate, Deserialize, Debug, Builder)]
pub struct Timings {
    redirect: Option<u32>,
    namelookup: Option<u32>,
    connection: Option<u32>,
    handshake: Option<u32>,
    response: Option<u32>,
    total: Option<u32>,
}

quick_error! {
    #[derive(Debug)]
    pub enum ChecksError {
        RequestFailed( cause : reqwest::Error){from()}
        JsonFailed( cause : serde_json::Error){from()}
    }
}
