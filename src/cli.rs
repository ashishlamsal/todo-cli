use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    name = "todo-cli",
    about = "A simple todo app written in Rust"
)]
pub struct CommandLineArgs {
    #[clap(subcommand)]
    pub command: Option<Action>,

    /// Sets a custom config file
    #[clap(
        short = 'f',
        long = "file",
        parse(from_os_str),
        value_name = "FILE_PATH"
    )]
    pub file_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Add a new task
    Add {
        #[clap()]
        task: String,
    },

    /// Remove a task
    Remove {
        #[clap()]
        position: usize,
    },

    /// Mark a task as done
    Done {
        #[clap()]
        position: usize,
    },

    /// List all tasks
    List,
}
