use serde::{Deserialize, Serialize};
use std::default::Default;
use structopt::StructOpt;
use std::str::FromStr;
use std::fmt::Debug;
use clap::ArgMatches;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, StructOpt)]
#[structopt(
name = "updown",
about = "A cli for http://updown.io",
rename_all = "snake"
)]
pub(crate) struct Updown {

    #[structopt(conflicts_with("opt"))]
    token_or_url : Option<String>,

    #[structopt(subcommand)]
    opt : Option<Subcommand>
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    Config {
        api_key: String,

        private_api_key: Option<String>,

        user_agent: Option<String>,
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
        page: Option<i32>,

        #[structopt(long)]
        results: Option<bool>,
    },

    Metrics {
        token: String,

        #[structopt(long)]
        from: Option<String>,

        #[structopt(long)]
        to: Option<String>,

        #[structopt(long)]
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

#[derive(Default)]
pub(crate) struct Parser {
    pub(crate) parse_errors : Vec<String>,
    pub(crate) successful_parse : bool
}

impl Parser {
    pub(crate) fn new() -> Parser {
        Parser {
            parse_errors: vec![],
            successful_parse : true
        }
    }

    pub(crate) fn successful_parse(&self) -> Result<(), CliError> {
        if !self.successful_parse {
            Err(CliError::BadArg(self.parse_errors.join("\n")))
        } else {
            Ok(())
        }
    }

    pub(crate) fn parse_value<T>(&mut self, key: &str, matches: &ArgMatches<'_>) -> Option<T>
        where T: FromStr, T::Err: Debug {
        let result = matches.value_of(key);
        match &result {
            Some(r) => {
                let v = r.parse::<T>();
                match v {
                    Ok(m) => Option::from(m),
                    Err(e) => {
                        self.parse_errors.push(format!("page ({} given)", matches.value_of("page").unwrap()));
                        self.successful_parse = false;
                        None
                    }
                }
            }
            _ => Option::None
        }
    }
}

quick_error! {

    /// Error specific to updown
    #[derive(Debug)]
    pub enum  CliError{
       BadArg (message : String) { display("Failed to parse value(s) : {}", message )}
    }
}

fn set<'a, T, F>(fields_to_functions : HashMap<&str, F>, matches : &ArgMatches<'_>)
    where F: for<'b> Fn(T) -> (),
          T: FromStr + 'a,
          T::Err: Debug + 'a {
    let mut parser: Parser = Parser::new();
    for entry in fields_to_functions {
        let v = parser.parse_value::<T>(entry.0, matches) ;
        if v.is_some() {
            entry.1(v.unwrap());
        }
    }
}