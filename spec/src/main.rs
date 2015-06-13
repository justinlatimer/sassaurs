#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;
extern crate glob;
extern crate littletest;

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
        None => "sass-spec/spec",
        Some(ref dir) => dir.as_ref()
    };

    if !args.flag_silent && !args.flag_tap {
        println!("Recursively searching under directory '{}' \
            for test files to test '{}' with.",
            directory,
            args.flag_command);
        println!("{}", version(&args.flag_command));
    }

    let path = std::path::Path::new(directory);
    let opts = tests::RunOptions {
        ignore_todo: args.flag_ignore_todo,
        command: args.flag_command
    };

    let runnables = tests::load(path, &opts);

    let runner = littletest::TestRunner::new(littletest::TestOptions {
        parallelism: Some(4)
    });
    runner.run(&runnables);
}

fn version(command: &str) -> String {
    let executable = command.split(' ').next().unwrap();
    match exec(executable, &["-v"]) {
        Some(output) => output,
        None => match exec(executable, &["-V"]) {
            Some(output) => output,
            None => panic!("Could not get version of {}", executable)
        }
    }
}

fn exec(command: &str, args: &[&str]) -> Option<String> {
    use std::process::Command;
    let mut wrapper = Command::new(command);
    match wrapper.args(args).output() {
        Ok(output) => match output.status.success() {
            true => Some(String::from_utf8(output.stdout).unwrap().to_string()),
            _ => None
        },
        Err(e) => panic!("failed to execute process: '{:?}' error: '{}'", wrapper, e)
    }
}
