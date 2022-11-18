use regex::Regex;
use std::process::Command;


// TODO maybe this could return an option too, like get_company_task_id_from_last_git_message?
pub fn get_company_task_id_from_current_git_branch() -> Result<String, String> {
    log::warn!("The task ID argument was not provided, attempting to fetch it from your current git branch");

    // Output to rust the current git branch
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .map_err(|_| {
            "forecast-cli could not run git command, are you in a git repository?".to_string()
        })?;

    // Get a String from the stdout and regex match the company task id
    let git_hash = String::from_utf8(output.stdout).unwrap();
    let regex = Regex::new("(T[0-9]*)").unwrap();

    let matches = regex.captures(git_hash.as_str()).ok_or_else(|| {
        "forecast-cli could not find a task id in your current git branch".to_string()
    })?;

    matches
        .get(0)
        .map(|r| r.as_str().to_string())
        .ok_or_else(|| {
            "forecast-cli could not find a task id in your current git branch".to_string()
        })
}

// Silently attempt to find a task id in the last git commit's message
pub fn get_company_task_id_from_last_git_message() -> Option<String> {
    log::warn!("Your git branch did not contain any task number, attempting to fetch it from your last git commit's message");

    // Output to rust the current git branch
    let output = Command::new("git")
        .args(["log", "-1", "--pretty=%B"])
        .output()
        .ok()?;

    // Get a String from the stdout and regex match the company task id
    let git_hash = String::from_utf8(output.stdout).unwrap();
    let regex = Regex::new("(T[0-9]*)").unwrap();

    let matches = regex.captures(git_hash.as_str())?;

    matches.get(0).map(|r| r.as_str().to_string())
}

pub fn format_task(task: &str) -> Option<i32> {
    let task_id_str: String = task.chars().skip(1).collect();

    let task_id: i32 = task_id_str.parse().ok()?;
    Some(task_id)
}

pub fn human_friendly_time_to_minutes(time: &str) -> Option<i32> {
    let mut convertible = time.split('h');
    let hours_str = convertible.next()?;
    let minutes_str = convertible.next()?;

    let hours: i32 = hours_str.parse().ok()?;
    let minutes: i32 = minutes_str.parse().unwrap_or(0);

    Some(minutes + hours * 60)
}
