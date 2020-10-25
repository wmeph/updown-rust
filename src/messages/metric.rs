use serde::{Deserialize, Serialize};
use validator::Validate;
use clap::ArgMatches;

/// Metrics represents the output of /api/checks/:token/metrics
/// Possible return values are a Metric message or an error message.
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Metrics {
    Error{ error : Option<String>},
    Metrics(Option<Message>)
}


#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Message {
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
#[builder(setter(strip_option))]
pub(crate) struct MetricsParams<'a> {
    #[serde(rename = "api-key")]
    api_key: &'a str,
    #[serde(skip)]
    pub(crate) token : &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    from : Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    to : Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    group: Option<&'a str>,
}

impl MetricsParams<'_> {
    pub(crate) fn parse<'a>(api_key: &'a str, matches: &'a ArgMatches<'_>) -> MetricsParams<'a> {
        let mut params = MetricsParamsBuilder::default();
        params.api_key(api_key);
        params.token(matches.value_of("token").unwrap());
        if let Some(from) = matches.value_of("from") { params.from(from); }
        if let Some(to) = matches.value_of("to") { params.to(to); }
        if let Some(group) = matches.value_of("group") { params.group(group); }
        params.build().unwrap()
    }
}
