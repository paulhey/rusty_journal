use chrono::Local;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created: DateTime<Utc> = Utc::now();
        Task { text, created }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created = self.created.with_timezone(&Local).format("%Y-%b-%d %T");
        write!(
            f,
            "{:<50} [{}]",
            self.text,
            created.to_string().to_uppercase()
        )
    }
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // Reset file handle to the start of the file
    file.seek(SeekFrom::Start(0))?;

    // Read the file contents as a Vec of Tasks
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Reset file handle to the start of the file
    file.seek(SeekFrom::Start(0))?;

    // Return the tasks
    Ok(tasks)
}

pub fn add_task(file_path: PathBuf, task: Task) -> Result<()> {
    // open the file_path
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Read the file contents as a Vec of Tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // add the new task to the end
    tasks.push(task);

    // Write the changes back to file
    serde_json::to_writer(file, &tasks)?;

    // Return OK empty set.
    Ok(())
}

pub fn complete_task(file_path: PathBuf, position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).write(true).open(file_path)?;

    // Read the file contents as a Vec of Tasks
    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    // Remove the entry
    if position == 0 || position > tasks.len() {
        // Position index is 1..N; where N is the number of tasks
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    } else {
        // remove the task from the list
        tasks.remove(position - 1);

        // reset the file size, as we now have fewer tasks
        file.set_len(0)?;

        // Write back to the file
        serde_json::to_writer(file, &tasks)?;

        // Return OK empty set.
        Ok(())
    }
}

pub fn list_tasks(file_path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).open(file_path)?;

    // Read the file contents as a Vec of Tasks
    let tasks: Vec<Task> = collect_tasks(&file)?;

    // Enumerate and display tasks, if any.
    if tasks.is_empty() {
        println!("There are no tasks stored.");
    } else {
        for (index, task) in tasks.iter().enumerate() {
            println!("[{}] : {}", index + 1, task);
        }
    }

    // Return OK empty set.
    Ok(())
}
