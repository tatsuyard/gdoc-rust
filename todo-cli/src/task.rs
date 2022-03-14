use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::Result;
use std::io::{Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file.
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vector of tasks.
    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    // Rewind the file after reading from it.
    file.seek(SeekForm::Start(0))?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks);
    Ok(());
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {}
