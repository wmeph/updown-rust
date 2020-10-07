use clap::ArgMatches;
use std::str::FromStr;
use std::fmt::Debug;

pub(crate) mod check;
pub(crate) mod downtime;
pub(crate) mod metric;

quick_error! {

    /// Error specific to updown
    #[derive(Debug)]
    pub enum MessageError {
        RequestFailed( cause : reqwest::Error){from()}
        JsonFailed( cause : serde_json::Error){from()}
        ParseFailed(cause : Vec<String>) {display("{}", "Parameter parsing failed: ".to_owned() + &cause.join(" "))}
    }
}

