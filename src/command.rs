use crate::client::Client;
use crate::config::Config;
use crate::messages::check::{Check, CheckParams};
use crate::messages::downtime::{DowntimeParams, Downtimes};
use crate::messages::metric::{Metrics, MetricsParams};
use crate::messages::MessageError;
use clap::ArgMatches;
use serde::export::fmt::Display;
use std::fmt::Debug;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "updown",
    about = "A cli for http://updown.io",
    rename_all = "snake"
)]
pub(crate) struct Updown {
    #[structopt(conflicts_with("opt"))]
    token_or_url: Option<String>,

    #[structopt(subcommand)]
    opt: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Config {
        api_key: String,

        private_api_key: String,

        user_agent: String,
    },

    All {},

    Check {
        token: String,

        #[structopt(long)]
        metrics: bool,
    },

    Downtimes {
        token: String,

        #[structopt(long)]
        page: Option<u32>,

        #[structopt(long)]
        results: Option<bool>,
    },

    Metrics {
        token: String,

        #[structopt(long)]
        from: Option<String>,

        #[structopt(long)]
        to: Option<String>,

        #[structopt(long, possible_values=&["time", "host"])]
        group: Option<String>,
    },

    Add {
        url: String,

        #[structopt(long)]
        alias: Option<String>,

        #[structopt(long)]
        last_status: Option<u16>,

        #[structopt(long)]
        uptime: Option<f32>,

        #[structopt(long)]
        down: Option<bool>,

        #[structopt(long)]
        down_since: Option<String>,

        #[structopt(long)]
        error: Option<String>,

        #[structopt(long)]
        period: Option<u32>,

        #[structopt(long)]
        apdex_t: Option<f32>,

        #[structopt(long)]
        string_match: Option<String>,

        #[structopt(long)]
        enabled: Option<bool>,

        #[structopt(long)]
        published: Option<bool>,

        #[structopt(long)]
        disabled_locations: Option<Vec<String>>,

        #[structopt(long)]
        last_check_at: Option<String>,

        #[structopt(long)]
        next_check_at: Option<String>,

        #[structopt(long)]
        mute_until: Option<String>, //?

        #[structopt(long)]
        favicon_url: Option<String>,

        //#[structopt(long)]
        //custom_headers: Option<HashMap<String, String>>,
        #[structopt(long)]
        http_verb: Option<String>,

        #[structopt(long)]
        http_body: Option<String>,
    },

    Update {
        token: String,

        #[structopt(long)]
        url: Option<String>,

        #[structopt(long)]
        alias: Option<String>,

        #[structopt(long)]
        last_status: Option<u16>,

        #[structopt(long)]
        uptime: Option<f32>,

        #[structopt(long)]
        down: Option<bool>,

        #[structopt(long)]
        down_since: Option<String>,

        #[structopt(long)]
        error: Option<String>,

        #[structopt(long)]
        period: Option<u32>,

        #[structopt(long)]
        apdex_t: Option<f32>,

        #[structopt(long)]
        string_match: Option<String>,

        #[structopt(long)]
        enabled: Option<bool>,

        #[structopt(long)]
        published: Option<bool>,

        #[structopt(long)]
        disabled_locations: Option<Vec<String>>,

        #[structopt(long)]
        last_check_at: Option<String>,

        #[structopt(long)]
        next_check_at: Option<String>,

        #[structopt(long)]
        mute_until: Option<String>, //?

        #[structopt(long)]
        favicon_url: Option<String>,

        //#[structopt(long)]
        //custom_headers: Option<HashMap<String, String>>,
        #[structopt(long)]
        http_verb: Option<String>,

        #[structopt(long)]
        http_body: Option<String>,
    },

    Delete {
        token: String,
    },
}

pub(crate) struct Parser<'a> {
    pub(crate) matches: &'a ArgMatches<'a>,
    pub(crate) parse_errors: Vec<String>,
    pub(crate) successful_parse: bool,
}

impl Parser<'_> {
    pub(crate) fn new<'a>(matches: &'a ArgMatches<'_>) -> Parser<'a> {
        Parser {
            matches,
            parse_errors: vec![],
            successful_parse: true,
        }
    }

    pub(crate) fn parse_value<T>(&mut self, key: &str) -> Option<T>
    where
        T: FromStr,
        T::Err: Debug + Display,
    {
        let result = self.matches.value_of(key);
        match result {
            Some(r) => {
                let v = r.parse::<T>();
                // Option::from(v.expect("Unsuccessful parse"))
                match v {
                    Ok(m) => Option::from(m),
                    Err(e) => {
                        self.parse_errors.push(format!(
                            "{} ({} given)",
                            e,
                            self.matches.value_of("page").unwrap()
                        ));
                        self.successful_parse = false;
                        None
                    }
                }
            }
            _ => Option::None,
        }
    }
}

quick_error! {

    /// Error specific to updown
    #[derive(Debug)]
    pub enum  CliError{
       BadArg (message : String ) { display("Failed to parse value(s) : {}", message )}
    }
}

pub(crate) async fn metrics(
    config: Config,
    subcommand_matches: &ArgMatches<'_>,
) -> Result<Metrics, MessageError> {
    let client = Client::new(
        config.api_key.as_str(),
        config.private_api_key.as_str(),
        config.user_agent.as_str(),
    );
    let params = MetricsParams::parse(client.api_key, subcommand_matches);
    client.metrics(&params).await
}

pub(crate) async fn downtimes(
    config: Config,
    subcommand_matches: &ArgMatches<'_>,
) -> Result<Downtimes, MessageError> {
    let client = Client::new(
        config.api_key.as_str(),
        config.private_api_key.as_str(),
        config.user_agent.as_str(),
    );
    let params = DowntimeParams::parse(client.api_key, &subcommand_matches);
    client.downtimes(&params).await
}

pub(crate) async fn add(
    config: Config,
    subcommand_matches: &ArgMatches<'_>,
) -> Result<Check, MessageError> {
    let client = Client::new(
        config.api_key.as_str(),
        config.private_api_key.as_str(),
        config.user_agent.as_str(),
    );
    let params = CheckParams::parse_update(client.api_key, subcommand_matches);
    match params {
        Ok(p) => client.update(&p).await,
        Err(e) => Err(MessageError::CommandFailed(e)),
    }
}
