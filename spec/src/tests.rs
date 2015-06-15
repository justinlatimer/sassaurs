use std::path;
use littletest;

pub struct RunOptions {
    pub ignore_todo: bool,
    pub command: String
}

pub struct TestCase<'o> {
    pub input_path: path::PathBuf,
    pub expected_path: path::PathBuf,
    pub opts: &'o RunOptions
}

impl<'o> TestCase<'o> {
    pub fn is_todo(&self) -> bool {
        match self.input_path.to_str() {
            Some(x) => x.contains("todo"),
            None => false
        }
    }
}

impl<'a> littletest::Runnable for TestCase<'a> {
    fn run(&self) -> littletest::TestResult {
        use littletest::{TestResult};

        if self.opts.ignore_todo && self.is_todo() {
            return TestResult::Skipped
        }

        TestResult::Pass
    }
}

pub fn load<'o>(spec_path: &path::Path,  opts: &'o RunOptions) -> Vec<Box<littletest::Runnable + Sync + 'o>> {
    use glob::glob;

    let input_file = "input.scss";
    let expected_file = "expected_output.css";

    let mut pattern = spec_path.to_path_buf();
    pattern.push("**");
    pattern.push(input_file);
    glob(pattern.to_str().unwrap())
        .unwrap()
        .map(|result| {
            let path = result.unwrap();
            let dir = path.parent().unwrap().to_path_buf();
            Box::new(TestCase {
                input_path: path,
                expected_path: dir.join(expected_file),
                opts: opts
            }) as Box<littletest::Runnable + Sync + 'o>
        })
        .collect()
}
