#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate quick_error;

extern crate validator_derive;

use structopt::StructOpt;
use clap::ArgMatches;
use std::collections::HashMap;

#[macro_use]
extern crate clap;

mod checks;
mod client;
mod downtime;

#[derive(Debug, StructOpt)]
#[structopt(name = "updown", about = "A cli for http://updown.io")]
enum Opt {

    Summary {

        token: String,

        #[structopt(short, long)]
        metrics: bool,
    },

    Downtimes {
        token: String,

        #[structopt(short, long)]
        page: Option<i32>,

        #[structopt(short, long)]
        results: Option<bool>,
    },

    Metrics {
        token: String,

        from: String,

        to: String,

        group: String,
    },
}

#[tokio::main]
async fn main() {
    Opt::from_args();

    let matches = Opt::clap().get_matches();
    let mut client = client::Client::new(
        "SZCehfLagQVXX5pcnZfi".to_string(),
        "ro-ATHcQvgqybDoLSodLzRA".to_string(),
        "".to_string(),
    );

    // println!("{:#?}", params(&matches.1.unwrap(), [("metrics", "value1")].iter().cloned().collect()));

    match matches.subcommand() {
        ("summary", Some(matches)) => {
            let metrics = matches.is_present("metrics");
            let token = matches.value_of("token").unwrap();
            println!("{:#?}", client.summary(token, metrics).await);
        },
        ("downtimes", Some(m)) => downtimes(&mut client, &m).await,
        ("metrics", Some(m)) => metrics(&mut client, &m).await,
        _ => unimplemented!()
    }
}

async fn downtimes(client: &mut client::Client, matches: &ArgMatches<'_>) {
    let params: Option<String>;
    if matches.is_present("page") {
        params = Option::from(format!("{}{}", "&page=", matches.value_of("page").unwrap()));
    } else {
        params = None;
    }
    let t = value_t!(matches.value_of("token"), String).unwrap();
    let resp2 = client.downtimes(t, params).await.unwrap();
    println!("{}", serde_json::to_string(&resp2).unwrap());
}

async fn summary(client: &mut client::Client, matches: &ArgMatches<'_>) {
    let m = matches.is_present("metrics");
    let t = value_t!(matches.value_of("token"), String).unwrap();
    // let resp2 = client.summary(t, m).await.unwrap();

}

async fn metrics(client: &mut client::Client, matches: &ArgMatches<'_>) {
    // let t = value_t!(matches.value_of("token"), String).unwrap();
    // let resp2 = client.check(t, m).await.unwrap();
    // println!("{}", serde_json::to_string(&resp2).unwrap());
}

fn params(matches : &ArgMatches<'_>, params: HashMap<&str, &str> ) -> String{
    let mut query_params= "".to_string();
    println!("{:#?}", matches);
    for p in params {
        if matches.is_present(p.0) {
             let p = "&".to_owned() + p.0 + "=" + matches.value_of(p.0).unwrap_or("false");
             query_params += &p;

        }
    }
    return query_params;
}

const CHECKS_URL: &'static str = "https://updown.io/api/checks";
