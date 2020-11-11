#[macro_use]
extern crate quick_error;

use client::Client;
use command::Updown;
use config::Config;

use crate::messages::check::CheckParams;
use confy::ConfyError;
use std::process::exit;
use structopt::StructOpt;
use validator::ValidationErrors;

extern crate clap;
extern crate exitcode;
extern crate validator;
#[macro_use]
extern crate derive_builder;

mod client;
mod command;
mod config;
mod messages;

/// This is a bit of a mish-mash and probably needs sorting out!
#[tokio::main]
async fn main() {
    let matches = Updown::clap().get_matches();
    let subcommand_name = matches.subcommand().0;

    if subcommand_name == "" {
        Updown::clap().print_help();
        exit(exitcode::NOINPUT);
    }

    let subcommand_matches = matches.subcommand().1.unwrap();
    if subcommand_name == "config" {
        let mut api_key;
        let mut private_api_key;
        let mut user_agent;

        match subcommand_matches.value_of("api-key") {
            Some(k) => {
                api_key = k;
            }

            None => {
                println!("No api key provided. Exiting.");
                exit(exitcode::CONFIG);
            }
        }

        match subcommand_matches.value_of("private-api-key") {
            Some(k) => {
                private_api_key = k;
            }

            None => {
                println!("No private (super) api key provided. Exiting.");
                exit(exitcode::CONFIG);
            }
        }

        match subcommand_matches.value_of("user-agent") {
            Some (a) => {
                user_agent = a;
            }
            None => {
                println!("User agent not provided. Using blank value");
                user_agent = "";
            }
        }

        let config = config::Config {
            api_key: api_key.to_string(),
            private_api_key: private_api_key.to_string(),
            user_agent: user_agent.to_string(),
        };
        match confy::store("updown-rust", config) {
            Ok(_c) => exit(exitcode::OK),
            Err(e) => {
                eprintln!("Failed to save config {}", e.to_string());
                exit(exitcode::IOERR)
            }
        };
    }

    let config;
    match Config::load_config() {
        Ok(c) => {
            config = c;
        }
        Err(e) => {
            eprintln!("Error {}", e);
            exit(exitcode::CONFIG);
        }
    }

    match subcommand_name {
        "all" => {
            let client = Client::new(
                config.api_key.as_str(),
                config.private_api_key.as_str(),
                config.user_agent.as_str(),
            );
            let result = client.all().await;
            let result = serde_json::to_string(&result.unwrap()).unwrap();
            println!("{}", result);
        }
        "check" => {
            let client = Client::new(
                config.api_key.as_str(),
                config.private_api_key.as_str(),
                config.user_agent.as_str(),
            );
            let metrics = subcommand_matches.is_present("metrics");
            let token = subcommand_matches.value_of("token").unwrap();
            let result =
                serde_json::to_string(&client.check(token, metrics).await.unwrap()).unwrap();
            println!("{}", result);
        }

        "downtimes" => {
            let result = command::downtimes(config, subcommand_matches).await;
            println!("{}", serde_json::to_string(&result.unwrap()).unwrap());
        }

        "metrics" => {
            let result = command::metrics(config, subcommand_matches).await;
            println!("{}", serde_json::to_string(&result.unwrap()).unwrap());
        }

        "add" => {
            println!(
                "{}",
                serde_json::to_string(&command::add(config, subcommand_matches).await.unwrap())
                    .unwrap()
            );
        }
        "update" => {
            let client = Client::new(
                config.api_key.as_str(),
                config.private_api_key.as_str(),
                config.user_agent.as_str(),
            );
            let params = CheckParams::parse_update(&client.api_key, &subcommand_matches).unwrap();

            let result = serde_json::to_string(&client.update(&params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        "delete" => {
            let client = Client::new(
                config.api_key.as_str(),
                config.private_api_key.as_str(),
                config.user_agent.as_str(),
            );
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
    }
}

const CHECKS_URL: &'static str = "https://updown.io/api/checks";
