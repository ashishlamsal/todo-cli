use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Error, ErrorKind, Result, Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    // Serialize and deserialize DateTime as Unix timestamp (seconds)
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = self
            .created_at
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Rewind file before reading from it
    file.seek(SeekFrom::Start(0))?;

    // Read existing tasks
    let tasks: Vec<Task> = match serde_json::from_reader(&mut BufReader::new(file)) {
        Ok(tasks) => tasks,                 // Deserialize existing tasks
        Err(e) if e.is_eof() => Vec::new(), // empty file
        Err(e) => Err(e)?,                  // other error
    };

    // Rewind file after reading from it
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn add_task(task: Task, file_path: PathBuf) -> Result<()> {
    // Open file in append mode
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)?;

    // Read existing tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Append new task to the end of the file
    tasks.push(task);
    serde_json::to_writer(&mut BufWriter::new(&file), &tasks)?;

    // List tasks
    list_tasks(file_path)?;

    Ok(())
}

pub fn complete_task(position: usize, file_path: PathBuf) -> Result<()> {
    // Open file in read and write mode
    let file = match OpenOptions::new().read(true).write(true).open(&file_path) {
        Ok(file) => file,
        Err(_) => Err(Error::new(
            ErrorKind::NotFound,
            r#"Try adding a task first. (-- add "my new task")"#,
        ))?,
    };

    // Read existing tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Remove task from the list
    if position == 0 || position > tasks.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Could not find task with given ID",
        ));
    }
    tasks.remove(position - 1);

    // Write updated tasks to the file
    file.set_len(0)?; // truncate file
    serde_json::to_writer(&mut BufWriter::new(&file), &tasks)?;

    // List tasks
    list_tasks(file_path)?;

    Ok(())
}

pub fn list_tasks(file_path: PathBuf) -> Result<()> {
    // Open file in read mode
    let file = match OpenOptions::new().read(true).open(&file_path) {
        Ok(file) => file,
        Err(_) => Err(Error::new(
            ErrorKind::NotFound,
            r#"Try adding a task first. (-- add "my new task")"#,
        ))?,
    };

    // Read existing tasks
    let tasks: Vec<Task> = collect_tasks(&file)?;

    // Print tasks
    if tasks.is_empty() {
        println!("No tasks!");
    } else {
        println!("{:<53} {}", "List of Tasks", "Date Added");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}
