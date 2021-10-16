use structopt::StructOpt;
mod cli;
use cli::{Action::*, CommandLineArgs};

mod tasks;
use tasks::Task;


fn main() {
  //
  let CommandLineArgs {
    action,
    filepath,
    debug
  } = CommandLineArgs::from_args();

  let file_path = filepath.expect("Failed to find journal file");

  if debug {
    println!("ARGS: <{:#?}> ", CommandLineArgs::from_args())
  } else {
    match action {
      Add { text } => tasks::add_task(file_path, Task::new(text)),
      Done { position } => tasks::complete_task(file_path, position),
      List => tasks::list_tasks(file_path)
    }.expect("Failed to perform action!")
  }
}
