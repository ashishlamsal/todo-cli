use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

mod cli;
mod tasks;

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let CommandLineArgs { action, file_path } = CommandLineArgs::from_args();

    // Get file path
    let file_path = file_path.unwrap_or_else(|| PathBuf::from("todo.json"));

    // Perform action
    match action {
        Add { task } => {
            // Create new task
            let task = Task::new(task);

            // Add task to the file
            tasks::add_task(task, file_path)?;
        }
        Done { position } => {
            // Complete task
            tasks::complete_task(position, file_path)?;
        }
        List => {
            // List tasks
            tasks::list_tasks(file_path)?;
        }
    }

    Ok(())
}
