mod cli;
mod tasks;

use anyhow::anyhow;
use clap::Parser;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_tasks_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".todos.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let CommandLineArgs { command, file_path } = CommandLineArgs::parse();

    // Get file path
    let file_path = file_path
        .or_else(find_default_tasks_file)
        .ok_or(anyhow!("Failed to find default tasks file."))?;

    // Perform action
    match command {
        Some(Add { task }) => {
            // Create new task
            let task = Task::new(task);

            // Add task to the file
            tasks::add_task(task, file_path)?;
        }
        Some(Remove { position }) => {
            // Remove task from the file
            tasks::remove_task(position, file_path)?;
        }
        Some(Done { position }) => {
            // Mark task as done
            tasks::complete_task(position, file_path)?;
        }
        Some(List) => {
            // List tasks
            tasks::list_tasks(file_path)?;
        }
        None => {
            // Print help
            println!(r#"Try `.\todo-cli.exe --help` for more information."#);
        }
    }

    Ok(())
}
