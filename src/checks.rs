use core::option::Option::{None, Some};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        15 | 60 | 120 | 3600 => Ok(()),
        _ => Err(ValidationError::new(concat!("Invalid value for period"))),
    }
}

impl Check {
    pub fn new() -> Check {
        Check {
            token: None,
            url: None,
            alias: None,
            last_status: None,
            uptime: None,
            down: None,
            down_since: None,
            error: None,
            period: None,
            apdex_t: None,
            string_match: None,
            enabled: None,
            published: None,
            disabled_locations: None,
            last_check_at: None,
            next_check_at: None,
            mute_until: None,
            favicon_url: None,
            custom_headers: None,
            http_verb: None,
            http_body: None,
            ssl: None,
            metrics: None,
        }
    }

    pub fn token<'a>(&'a mut self, token: String) -> &'a mut Check {
        self.token = Some(token);
        self
    }

    pub fn url<'a>(&'a mut self, url: String) -> &'a mut Check {
        self.url = Some(url);
        self
    }

    pub fn period<'a>(&'a mut self, period: u32) -> &'a mut Check {
        self.period = Some(period);
        self
    }

    pub fn apdex_t<'a>(&'a mut self, apdex_t: f32) -> &'a mut Check {
        self.apdex_t = Some(apdex_t);
        self
    }

    pub fn enabled<'a>(&'a mut self, enabled: bool) -> &'a mut Check {
        self.enabled = Some(enabled);
        self
    }

    pub fn published<'a>(&'a mut self, published: bool) -> &'a mut Check {
        self.published = Some(published);
        self
    }

    pub fn alias<'a>(&'a mut self, alias: String) -> &'a mut Check {
        self.alias = Some(alias);
        self
    }

    pub fn string_match<'a>(&'a mut self, string_match: String) -> &'a mut Check {
        self.string_match = Some(string_match);
        self
    }

    pub fn mute_until<'a>(&'a mut self, mute_until: String) -> &'a mut Check {
        self.mute_until = Some(mute_until);
        self
    }

    pub fn http_verb<'a>(&'a mut self, http_verb: String) -> &'a mut Check {
        self.http_verb = Some(http_verb);
        self
    }

    pub fn http_body<'a>(&'a mut self, http_body: String) -> &'a mut Check {
        self.http_body = Some(http_body);
        self
    }

    pub fn disabled_locations<'a>(&'a mut self, disabled_locations: Vec<String>) -> &'a mut Check {
        self.disabled_locations = Some(disabled_locations);
        self
    }

    pub fn custom_headers<'a>(
        &'a mut self,
        custom_headers: HashMap<String, String>,
    ) -> &'a mut Check {
        self.custom_headers = Some(custom_headers);
        self
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

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Metrics {
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

quick_error! {
    #[derive(Debug)]
    pub enum ChecksError {
        RequestFailed( cause : reqwest::Error){from()}
        JsonFailed( cause : serde_json::Error){from()}
    }
}
