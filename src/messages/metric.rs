use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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
struct MetricParams {
    // from [time] = 2020-09-03 20:59:51 UTC
    // Start time, default to 1 month ago ­· supported formats ⇣
    // to [time] = 2020-10-03 22:59:51 +0200
    // End time, default to now ­· supported formats ⇣
    // group [symbol]
    // Group data by 'time' (hour) or 'host' (location)

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
