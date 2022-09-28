use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Action {
  /// Append a task to the journal file
  Add {
    /// The task description text
    #[clap()]
    text: String,
  },

  /// Remove an entry from the journal file by position
  Done {
    /// Task position number
    #[clap()]
    position: usize,
  },
  /// List all tasks in the journal file
  List,
}

#[derive(Debug, Parser)]
#[clap(
  name = "Rusty Journal",
  about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
  #[clap(subcommand)]
  pub action: Action,

  /// Use a non-default journal file path
  #[clap(value_parser, short, long)]
  pub filepath: Option<PathBuf>,

  /// Activate Debug Mode
  #[clap(short, long)]
  pub debug: bool,
}
