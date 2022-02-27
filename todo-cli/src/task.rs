use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::io::Result;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Result, Seek, SeekFrom};

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
    let mut file = OpenOptions::new().read(true).write(true).create(true);
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> { ... }

pub fn list_tasks(journal_path: PathBuf) -> Result<()> { ... }
