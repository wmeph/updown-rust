#[macro_use]
extern crate quick_error;

use crate::checks::Check;
use crate::validator::Validate;
use clap::ArgMatches;

use crate::downtime::DowntimeParams;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;
use std::process::exit;
use structopt::StructOpt;
use validator::ValidationErrors;
use crate::client::Client;

#[macro_use]
extern crate clap;
#[macro_use]
extern crate validator;
extern crate exitcode;
#[macro_use]
extern crate derive_builder;

mod checks;
mod client;
mod downtime;

#[derive(Serialize, Deserialize, Default)]
struct Config {
    api_key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    private_api_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "updown",
    about = "A cli for http://updown.io",
    rename_all = "snake"
)]
enum Opt {
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

#[tokio::main]
async fn main() {
    Opt::from_args();
    let matches = Opt::clap().get_matches();


    match matches.subcommand() {
        ("config", Some(matches)) => match matches.value_of("api_key") {
            Some(k) => {
                let api_key = k.to_string();
                let config: Config = Config {
                    api_key: api_key,
                    private_api_key: None,
                    user_agent: None,
                };
                confy::store("updown-rust", config);
            }
            None =>  {
                println!("No api key provided. Exiting.");
                exit(exitcode::CONFIG);
            },
        },

        ("all", Some(matches)) => {
            let client = get_client();
            let result = serde_json::to_string(&client.all().await.unwrap()).unwrap();
            println!("{}", result);
        }
        ("check", Some(matches)) => {
            let client = get_client();
            let metrics = matches.is_present("metrics");
            let token = matches.value_of("token").unwrap();
            let result =
                serde_json::to_string(&client.check(token, metrics).await.unwrap()).unwrap();
            println!("{}", result);
        }
        ("downtimes", Some(matches)) => {
            let client = get_client();
            let token = matches.value_of("token").unwrap();
            let params = DowntimeParams::parse(client.api_key.as_str(), matches);
            let _resp = client.downtimes(token, &params).await;
            // let result = serde_json::to_string(&client.downtimes(token, &params).await.unwrap()).unwrap();
        }
        // ("downtimes", Some(m)) => downtimes(&mut client, &m).await,
        // ("metrics", Some(m)) => metrics(&mut client, &m).await,
        ("add", Some(matches)) => {
            let client = get_client();
            let url = matches.value_of("url").unwrap();
            let params = params(&matches);
            let result = serde_json::to_string(&client.add(url, &params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        ("update", Some(matches)) => {
            let client = get_client();
            let token = matches.value_of("token").unwrap();
            let params = params(&matches);
            let result =
                serde_json::to_string(&client.update(token, &params).await.unwrap()).unwrap();
            println!("{}", result);
        }
        ("delete", Some(matches)) => {
            let client = get_client();
            let token = matches.value_of("token").unwrap();
            let result = serde_json::to_string(&client.delete(token).await.unwrap()).unwrap();
            println!("{}", result);
        }
        _ => unimplemented!(),
    }
}

fn get_client() -> Client {
    let config: Config;

    match confy::load("updown-rust") {
        Ok(c) => config = c,
        Err(_e) => {
            println!("No api key provided. Exiting.");
            exit(exitcode::CONFIG);
        }
    }

    let mut client = Client::new(
        config.api_key.to_string(),
        "ro-ATHcQvgqybDoLSodLzRA".to_string(),
        "".to_string(),
    );
    client
}

fn print_errors(e: ValidationErrors) {
    for (k, v) in e.field_errors() {
        println!(
            "Validation error for field {:#?} : {} ({} given)",
            k, v[0].code, v[0].params["value"]
        );
    }
}


// TODO Move this to CheckParams
fn params(matches: &ArgMatches<'_>) -> Check {
    let mut check = checks::Check::new();
    if matches.is_present("url") {
        check.url(matches.value_of("url").unwrap().parse().unwrap());
    }
    if matches.is_present("period") {
        let period = matches.value_of("period").unwrap().parse::<u32>().unwrap();
        check.period(period);
    }
    if matches.is_present("apdex_t") {
        let apdex_t = matches.value_of("apdex_t").unwrap().parse::<f32>().unwrap();
        check.apdex_t(apdex_t);
    }
    if matches.is_present("enabled") {
        check.enabled(
            matches
                .value_of("enabled")
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        );
    }
    if matches.is_present("published") {
        check.published(
            matches
                .value_of("published")
                .unwrap()
                .parse::<bool>()
                .unwrap(),
        );
    }
    if matches.is_present("alias") {
        check.alias(matches.value_of("alias").unwrap().parse().unwrap());
    }
    if matches.is_present("string_match") {
        check.string_match(matches.value_of("string_match").unwrap().parse().unwrap());
    }
    if matches.is_present("mute_until") {
        check.mute_until(matches.value_of("mute_until").unwrap().parse().unwrap());
    }
    if matches.is_present("http_verb") {
        check.http_verb(matches.value_of("http_verb").unwrap().parse().unwrap());
    }
    if matches.is_present("http_body") {
        check.http_body(matches.value_of("http_body").unwrap().parse().unwrap());
    }
    if matches.is_present("disabled_locations") {
        unimplemented!()
        //check.disabled_locations(matches.value_of("disabled_locations").unwrap().parse().unwrap());
    }
    if matches.is_present("custom_headers") {
        unimplemented!()
        // check.custom_headers(matches.value_of("custom_headers").unwrap().parse().unwrap());
    }

    let result = check.validate();
    match result {
        Ok(()) => {
            println!("valid!");
            return check;
        }
        Err(e) => {
            print_errors(e);
            exit(exitcode::DATAERR);
        }
    };
}

const CHECKS_URL: &'static str = "https://updown.io/api/checks";
