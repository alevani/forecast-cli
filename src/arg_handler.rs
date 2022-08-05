use clap::{value_t, App as CLAPApp, Arg};

pub fn get_values() -> (String, String) {
    let args = CLAPApp::new("HOWTO")
        .version("1.0")
        .arg(
            Arg::with_name("task")
                .short("t")
                .long("task")
                .help("Task number e.g: T15115")
                .value_name("Task number")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .help("Number of hours to log e.g: 0h25m")
                .value_name("Number of hour to log")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Quit the CLI if the value were not present
    let task_number = value_t!(args, "task", String).unwrap_or_else(|e| e.exit());
    let number_of_hours = value_t!(args, "log", String).unwrap_or_else(|e| e.exit());

    (task_number, number_of_hours)
}
