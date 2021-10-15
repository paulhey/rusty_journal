mod cli;
use structopt::StructOpt;

fn main() {
  //
  let myargs = cli::CommandLineArgs::from_args();
  if myargs.debug {
    println!("ARGS: <{:#?}> ", myargs);
  } else {
    println!("Hello, world! Rusty Journal!");
  }
}
