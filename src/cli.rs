use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Action {
    /// Add a new task
    Add {
        #[structopt()]
        task: String,
    },

    /// Mark a task as done
    Done {
        #[structopt()]
        position: usize,
    },

    /// List all tasks
    List,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "todo-cli", about = "A simple todo app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use different file for storing tasks
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    pub file_path: Option<PathBuf>,
}
