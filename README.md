# rusty_journal

This is a tutorial program done from [here](https://docs.microsoft.com/en-us/learn/modules/rust-create-command-line-program/).

This has recently been updated to use CLAP v4.0 and a tweaked version of chrono, so it diverges a bit from the original tutorial.

## Usage

```terminal
Rusty Journal 0.2.8
A command-line to-do app written in Rust

Usage: rj [OPTIONS] <COMMAND>

Commands:
  add   Append a task to the journal file
  done  Remove an entry from the journal file by position
  list  List all tasks in the journal file
  help  Print this message or the help of the given subcommand(s)

Options:
  -f, --filepath <FILEPATH>  Use a non-default journal file path
  -d, --debug                Activate Debug Mode
  -h, --help                 Print help information
  -V, --version              Print version information
 ```

## Example

```terminal
❯ rj -f test.json list
[1] : buy milk                                           [2021-OCT-15 17:21:16]
[2] : Go home                                            [2021-OCT-15 17:21:24]
[3] : Finish doing work                                  [2022-MAR-02 14:12:02]

❯ rj -f test.json add "Do more work"

❯ rj -f test.json list
[1] : buy milk                                           [2021-OCT-15 17:21:16]
[2] : Go home                                            [2021-OCT-15 17:21:24]
[3] : Finish doing work                                  [2022-MAR-02 14:12:02]
[4] : Do more work                                       [2022-OCT-05 17:06:28]

❯ rj -f test.json done 4

❯ rj -f test.json list
[1] : buy milk                                           [2021-OCT-15 17:21:16]
[2] : Go home                                            [2021-OCT-15 17:21:24]
[3] : Finish doing work                                  [2022-MAR-02 14:12:02]

```
