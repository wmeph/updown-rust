use clap::ArgMatches;
use std::str::FromStr;
use std::fmt::Debug;
use crate::command::CliError;

pub(crate) mod check;
pub(crate) mod downtime;
pub(crate) mod metric;

quick_error! {

    /// Error specific to updown
    #[derive(Debug)]
    pub enum MessageError {
        RequestFailed( cause : reqwest::Error){from()}
        JsonFailed( cause : serde_json::Error){from()}
        CommandFailed(cause : CliError) {display("{}", cause.to_string())}
    }
}

