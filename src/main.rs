#[macro_use]
extern crate quick_error;

use command::Updown;
use command::config;
use client::Client;
use config::Config;

use crate::messages::check::CheckParams;
use std::process::exit;
use structopt::StructOpt;
use validator::ValidationErrors;
use confy::ConfyError;
use crate::messages::metric::MetricsParams;
use messages::metric::MetricsParamsBuilder;


extern crate clap;
extern crate exitcode;
extern crate validator;
#[macro_use]
extern crate derive_builder;

mod command;
mod client;
mod config;
mod messages;

/// This is a bit of a mish-mash and probably needs sorting out!
#[tokio::main]
async fn main() {
    let matches = Updown::clap().get_matches();
    let subcommand_name = matches.subcommand().0;
    if subcommand_name == "config" {
        match matches.value_of("api-key") {
            Some(k) => {
                let api_key = k;
                let config = config::Config {
                    api_key: api_key.to_string(),
                    private_api_key: None,
                    user_agent: None,
                };
                confy::store("updown-rust", config);
                exit(exitcode::OK);
            }
            None => {
                println!("No api key provided. Exiting.");
                exit(exitcode::CONFIG);
            }
        }
    }
    
    if subcommand_name == "" {
        Updown::clap().print_help();
        exit(exitcode::NOINPUT);
    }

    let mut config;
    match Config::load_config(){
        Ok(c) => {
            config = c;
        }
        Err(e) => {
            eprintln!("Error {}", e);
            exit(exitcode::CONFIG);
        }
    }
    
    let subcommand_matches = matches.subcommand().1.unwrap();
    match subcommand_name {

        "all" => {
            let client = Client::new(config.api_key.as_str(), config.private_api_key, config.user_agent);
            let result = client.all().await;
            let result = serde_json::to_string(&result.unwrap()).unwrap();
            println!("{}", result);
        }
        "check" => {
            let client = Client::new(config.api_key.as_str(), config.private_api_key, config.user_agent);
            let metrics = subcommand_matches.is_present("metrics");
            let token = subcommand_matches.value_of("token").unwrap();
            let result =
                serde_json::to_string(&client.check(token, metrics).await.unwrap()).unwrap();
            println!("{}", result);
        }

        "downtimes" => {
            let result = command::downtimes(config, subcommand_matches).await;
            println!("{}", serde_json::to_string(&result.unwrap() ).unwrap());
        }

        "metrics" => {
            let result = command::metrics(config, subcommand_matches).await;
            println!("{}", serde_json::to_string(&result.unwrap() ).unwrap());
        }

        "add" => {
            println!("{}", serde_json::to_string(&command::add(config, subcommand_matches).await.unwrap()).unwrap());
        }
        "update" => {


            let client = Client::new(config.api_key.as_str(), config.private_api_key, config.user_agent);
            let params = CheckParams::parse_update(&client.api_key, &subcommand_matches)
                .unwrap();

            let result =
                serde_json::to_string(&client.update(&params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        "delete" => {
            let client = Client::new(config.api_key.as_str(), config.private_api_key, config.user_agent);
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

const CHECKS_URL: &'static str = "https://updown.io/api/checks";