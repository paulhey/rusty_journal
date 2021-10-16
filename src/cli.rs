use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
  /// Append a task to the journal file
  Add {
    /// The task description text
    #[structopt()]
    text: String,
  },

  /// Remove an entry from the journal file by position
  Done {
    /// Task position number
    #[structopt()]
    position: usize,
  },
  /// List all tasks in the journal file
  List,
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Rusty Journal",
  about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub action: Action,

  /// Use a non-default journal file path
  #[structopt(parse(from_os_str), short, long)]
  pub filepath: Option<PathBuf>,

  /// Activate Debug Mode
  #[structopt(short, long)]
  pub debug: bool,
}
