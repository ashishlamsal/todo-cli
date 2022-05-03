use cli::{Action::*, CommandLineArgs};
// use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

mod cli;
mod tasks;

fn find_default_tasks_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".todos.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let CommandLineArgs { action, file_path } = CommandLineArgs::from_args();

    // Get file path
    let file_path = file_path.or_else(find_default_tasks_file).unwrap();
    println!("Using: {}", file_path.display());

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
