use std::process::exit;

use chrono::Local;
use forecastapp_api::{self, forecastapp_models::TimeRegistrationBody, ForecastAppApi};
pub mod arg_handler;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // let api = ForecastAppApi::new();

    // let (raw_task, raw_time) = arg_handler::get_values();

    // let task_id = match format_task(raw_task) {
    //     Some(id) => id,
    //     None => {
    //         println!("The provided task id is not valid");
    //         exit(1);
    //     }
    // };

    // let time = match human_friendly_time_to_minutes(raw_time) {
    //     Some(time) => time,
    //     None => {
    //         println!("The provided time format is not valid");
    //         exit(1);
    //     }
    // };

    // let task = match api.get_task_id_by_company_task_id(task_id).await {
    //     Ok(task) => task,
    //     Err(_) => {
    //         println!("The task could not be found");
    //         exit(1);
    //     }
    // };

    // let time = TimeRegistrationBody {
    //     date: Local::today().format("%Y-%m-%d").to_string(),
    //     person: 261442,
    //     task: task.id,
    //     time_registered: time,
    // };

    // let _ = api.send_time_registration(time).await;

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //  terminal.draw(|f| {
    //      let size = f.size();
    //      let block = Block::default()
    //          .title(" Forecast CLI ")
    //          .borders(Borders::ALL);
    //      f.render_widget(block, size);
    //  })?;
    terminal.draw(|f| ui(f))?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

use tui::{backend::Backend, Frame};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let description_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());

    let logged_as_block = Block::default().title(" Logged as ").borders(Borders::ALL);
    f.render_widget(logged_as_block, vertical_chunks[0]);

    let overview_block = Block::default().title(" Overview ").borders(Borders::ALL);
    f.render_widget(overview_block, vertical_chunks[1]);

    let projects_block = Block::default().title(" Projects ").borders(Borders::ALL);
    f.render_widget(projects_block, vertical_chunks[2]);

    let tasks_block = Block::default().title(" Tasks ").borders(Borders::ALL);
    f.render_widget(tasks_block, vertical_chunks[3]);

    let description_block = Block::default()
        .title(" Description ")
        .borders(Borders::ALL);
    f.render_widget(description_block, description_chunk[0]);
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
