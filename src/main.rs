use home::home_dir;
use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
use cli::{Action::*, CommandLineArgs};

mod tasks;
use tasks::Task;

fn get_default_filepath() -> Option<PathBuf> {
  home_dir().map(|mut path| {
    path.push(".rusty_journal.json");
    path
  })
}

fn main() -> anyhow::Result<()> {
  //
  let CommandLineArgs {
    action,
    filepath,
    debug,
  } = CommandLineArgs::from_args();

  let file_path = filepath
    .or_else(get_default_filepath)
    .ok_or(anyhow!("Failed to find journal file"))?;
    // .expect("Failed to find journal file");

  if debug {
    println!("ARGS: <{:#?}> ", CommandLineArgs::from_args());
    Ok(())
  } else {
    match action {
      Add { text } => tasks::add_task(file_path, Task::new(text)),
      Done { position } => tasks::complete_task(file_path, position),
      List => tasks::list_tasks(file_path),
    }?;
    // .expect("Failed to perform action!")
    Ok(())
  }
}
