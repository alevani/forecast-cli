use std::process::exit;

use chrono::Local;
use forecastapp_api::{self, forecastapp_models::TimeRegistrationBody, ForecastAppApi};
pub mod arg_handler;

#[tokio::main]
async fn main() {
    let api = ForecastAppApi::new();

    let (raw_task, raw_time) = arg_handler::get_values();

    let task_id = match format_task(raw_task) {
        Some(id) => id,
        None => {
            println!("The provided task id is not valid");
            exit(1);
        }
    };

    let time = match human_friendly_time_to_minutes(raw_time) {
        Some(time) => time,
        None => {
            println!("The provided time format is not valid");
            exit(1);
        }
    };

    let task = match api.get_task_id_by_company_task_id(task_id).await {
        Ok(task) => task,
        Err(_) => {
            println!("The task could not be found");
            exit(1);
        }
    };

    let time = TimeRegistrationBody {
        date: Local::today().format("%Y-%m-%d").to_string(),
        person: 261442,
        task: task.id,
        time_registered: time,
    };

    let _ = api.send_time_registration(time).await;
}

fn format_task(task: String) -> Option<i32> {
    let task_id_str: String = task.chars().skip(1).collect();

    let task_id: i32 = task_id_str.parse().ok()?;
    Some(task_id)
}

fn human_friendly_time_to_minutes(time: String) -> Option<i32> {
    let mut convertible = time.split("h");
    let hours_str = convertible.next()?;
    let minutes_str = convertible.next()?.split("m").next()?;

    let hours: i32 = hours_str.parse().ok()?;
    let minutes: i32 = minutes_str.parse().ok()?;

    Some(minutes + hours * 60)
}
