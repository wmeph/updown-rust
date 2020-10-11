#[macro_use]
extern crate quick_error;

use crate::validator::Validate;
use clap::ArgMatches;

use cli::Updown;
use client::Client;

use crate::messages::check::{Check, CheckParams};
use crate::messages::downtime::DowntimeParams;
use crate::messages::metric::{Metrics, MetricsParams};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;
use std::process::exit;
use structopt::StructOpt;
use validator::ValidationErrors;
use std::str::FromStr;
use crate::config::Config;
use confy::ConfyError;
use std::error::Error;

extern crate clap;
extern crate exitcode;
extern crate validator;
#[macro_use]
extern crate derive_builder;

mod cli;
mod client;
mod config;
mod messages;

/// This is a bit of a mish-mash and probably needs sorting out!
#[tokio::main]
async fn main() {
    let matches = Updown::clap().get_matches();

    let subcommand_name = matches.subcommand().0;
    if subcommand_name == "" {
        println!("Nowt");
        if matches.is_present("token_or_url")  {
            println!("token");
            exit(exitcode::OK);
        }
        else {
            Updown::clap().print_help();
            exit(exitcode::NOINPUT)
        }
    }

    let subcommand_matches = matches.subcommand().1.unwrap();
    match subcommand_name {
        "config" => match subcommand_matches.value_of("api-key") {
            Some(k) => {
                let api_key = k.to_string();
                let config = config::Config {
                    api_key,
                    private_api_key: None,
                    user_agent: None,
                };
                confy::store("updown-rust", config);
            }
            None => {
                println!("No api key provided. Exiting.");
                exit(exitcode::CONFIG);
            }
        },

        "all" => {
            let client = Client::from_config().unwrap();
            let result = serde_json::to_string(&client.all().await.unwrap()).unwrap();
            println!("{}", result);
        }
        "check" => {
            let client = Client::from_config().unwrap();
            let metrics = subcommand_matches.is_present("metrics");
            let token = subcommand_matches.value_of("token").unwrap();
            let result =
                serde_json::to_string(&client.check(token, metrics).await.unwrap()).unwrap();
            println!("{}", result);
        }

        "downtimes" => {
            let client = Client::from_config().unwrap();
            let token = subcommand_matches.value_of("token").unwrap();
            let params = DowntimeParams::parse(&client.api_key, &subcommand_matches);
            if params.is_err() {
                    println!("{}", params.err().unwrap().to_string());
                    exit(exitcode::DATAERR)
            }
            let result =
                serde_json::to_string(&client.downtimes(token, &params.unwrap()).await.unwrap()).unwrap();
            println!("{}", result);
            // let result = serde_json::to_string(&client.downtimes(token, &params).await.unwrap()).unwrap();
        }

        "metrics" => {
            let result = cli::metrics(subcommand_matches).await;
            println!("{}", serde_json::to_string(&result.unwrap() ).unwrap());
        }


        // ("metrics", Some(m)) => metrics(&mut client, &m).await,

        "add" => {
            let client = Client::from_config().unwrap();
            let url = subcommand_matches.value_of("url").unwrap();
            let params =
                CheckParams::parse_add(&client.api_key, url.to_string(), &subcommand_matches).unwrap();
            let result = serde_json::to_string(&client.add(&params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        "update" => {
            let client = Client::from_config().unwrap();
            let token = subcommand_matches.value_of("token").unwrap();
            let params = CheckParams::parse_update(&client.api_key, &subcommand_matches)
                .unwrap();

            let result =
                serde_json::to_string(&client.update(token, &params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        "delete" => {
            let client = Client::from_config().unwrap();
            let token = subcommand_matches.value_of("token").unwrap();
            let result = serde_json::to_string(&client.delete(token).await.unwrap()).unwrap();
            println!("{}", result);
        }

        _ => unimplemented!(),
    }
}

quick_error! {

    /// Error specific to updown
    #[derive(Debug)]
    pub enum UpdownError {
        MessageFailed (cause : messages::MessageError){from()}
        ValidationFailed (cause : ValidationErrors){}
        ConfigurationFailed (cause : ConfyError){from()}
        // RequestFailed( cause : reqwest::Error){from()}
        // JsonFailed( cause : serde_json::Error){from()}
    }
}

///TODO This should probably be handled by a CLI-specific error type
fn print_errors(e: ValidationErrors) {
    for (k, v) in e.field_errors() {
        println!(
            "Validation error for field {:#?} : {} ({} given)",
            k, v[0].code, v[0].params["value"]
        );
    }
}

const CHECKS_URL: &'static str = "https://updown.io/api/checks";