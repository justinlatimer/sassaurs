#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

docopt!(pub Args derive Debug, "
Usage: sassaurs_spec [options] [<spec-dir>]
       sassaurs_spec --help

Options:
  -c COMMAND, --command COMMAND
                     Sets a specific binary to run
                     [default: sassc]
  -s, --skip         Skip tests that fail to exit successfully.
  -t, --tap          Output TAP compatible report.
  -v, --verbose      Run verbosely.
  --limit NUMBER     Limit the number of tests run to this positive integer.
  --filter PATTERN   Run tests that match the pattern you provide
  --ignore-todo      Skip any folder named 'todo'.
  --silent           Don't show any logs.
  --nuke             Write a new expected_output for every test.
  --unexpected-pass  When running the todo tests, flag as an error when a test
                     passes which is marked as todo.
",
flag_limit: Option<usize>,
flag_filter: Option<String>,
arg_spec_dir: Option<String>);

fn main() {
    let args: Args = Args::docopt()
        .decode()
        .unwrap_or_else(|e| e.exit());
}
