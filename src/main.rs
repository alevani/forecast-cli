use clap::{value_t, App as CLAPApp, Arg};
use forecastapp_api::{self, ForecastAppApi};

fn main() {
    let api = ForecastAppApi::new();
    let args = CLAPApp::new("HOWTO")
        .version("1.0")
        .arg(
            Arg::with_name("task")
                .short("t")
                .long("task")
                .value_name("TASK")
                .help("Task number")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .value_name("LOG")
                .help("Number of hour to log")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    println!("Hello, world!");
}
