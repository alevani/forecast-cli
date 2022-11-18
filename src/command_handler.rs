use crate::utils::{
    self, format_task, get_company_task_id_from_current_git_branch, human_friendly_time_to_minutes, get_company_task_id_from_last_git_message
};

pub async fn undo_handler() -> Result<(), String> {
    // Henrik's endpoint will return 200 even though their might be nothing to undo
    if delete_last_time_registration().await?.is_success() {
        println!("Undone last time registration");
    }

    Ok(())
}

pub async fn log_handler(
    ftime: String,
    unfmt_company_task_id: Option<String>,
    message: Option<String>,
) -> Result<(), String> {
    let time = human_friendly_time_to_minutes(&ftime)
        .ok_or_else(|| "The provided time format is not valid".to_string())?;

    let company_task_id = unfmt_company_task_id
        .map_or_else(get_company_task_id_from_current_git_branch, Ok)
        .ok()
        .map_or_else(get_company_task_id_from_last_git_message, Some)
        .ok_or("All attempts to fetch a task id failed")?;

    let task_id = format_task(&company_task_id)
        .ok_or_else(|| "The provided task id is not valid".to_string())?;

    let task = get_task_id_by_company_task_id(&task_id)
        .await
        .map_err(|err| format!("forecast-cli could not find your task on Forecast: {err}"))?;

    let time_reg = TimeReg {
        minutes: time,
        notes: message,
        project: None,
        task: Some(task[0].id),
    };

    if send_time_registration(&time_reg).await?.is_success() {
        println!("\nLogged {} on task {}", ftime, company_task_id);
    }

    Ok(())
}

pub fn open_handler(ctid: Option<String>) -> Result<(), String> {
    let company_task_id = ctid
        .map_or_else(get_company_task_id_from_current_git_branch, Ok)
        .ok()
        .map_or_else(get_company_task_id_from_last_git_message, Some)
        .ok_or("All attempts to fetch a task id failed")?;

    webbrowser::open(&format!("https://app.forecast.it/{company_task_id}")).ok();

    Ok(())
}

pub async fn today_handler() -> Result<(), String> {
    let time_regs = erp::get_todays_time_reg().await?;
    time_regs.iter().enumerate().for_each(|(i, time_reg)| {
        println!(
            "{}. [{}]: {} ({} minutes)
{}",
            i + 1,
            time_reg.project,
            time_reg.task,
            time_reg.minutes,
            time_reg.notes
        )
    });

    Ok(())
}
