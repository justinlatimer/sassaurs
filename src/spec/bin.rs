#![feature(plugin)]
#![plugin(docopt_macros)]
#![plugin(regex_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate glob;
extern crate littletest;
extern crate regex;

mod adapter;
use adapter::{Adapter,ExecutableAdapter};
mod tests;

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

    let directory = match args.arg_spec_dir {
        None => "spec",
        Some(ref dir) => dir.as_ref()
    };

    let engine = Box::new(ExecutableAdapter::new(&args.flag_command));

    if !args.flag_silent && !args.flag_tap {
        println!("Recursively searching under directory '{}' \
            for test files to test '{}' with.",
            directory,
            args.flag_command);
        println!("{}", engine.version());
    }

    let path = std::path::Path::new(directory);
    let opts = tests::RunOptions {
        ignore_todo: args.flag_ignore_todo,
        engine: engine as Box<Adapter>
    };

    let runnables = tests::load(path, &opts);
    let limited = match args.flag_limit {
        Some(count) => runnables.into_iter().take(count).collect(),
        _ => runnables
    };

    let runner = littletest::TestRunner::new(littletest::TestOptions {
        parallelism: Some(4)
    });
    runner.run(&limited);
}
