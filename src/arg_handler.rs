use clap::{Parser, Subcommand};

/// Forecast CLI
#[derive(Parser, Debug)]
#[command(next_line_help = true)]
pub struct LogArgs {
    /// The command you wish to execute
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Login to Forecast with your Adapt Google account
    Login,

    /// Logout from the CLI
    Logout,

    /// Undo your last time registration (only work after a log)
    Undo,

    /// Fetch a list of today's registered time
    Today,

    /// Log your time spent on a task
    Log {
        /// The time to register with the format <Number of hours>h<Optional: Number of minutes>. Examples: 2h or 2h45
        time: String,

        /// Task ID (e.g: T10019), will attempt to fetch the ID from your current git branch if not provided
        #[arg(short, long)]
        task: Option<String>,

        /// A short message about what you did on the task
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Open a task in your web browser
    Open {
        /// Task ID (e.g: T10019), will attempt to fetch the ID from your current git branch if not provided
        #[arg(short, long)]
        task: Option<String>,
    },
}
