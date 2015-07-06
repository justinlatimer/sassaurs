use std::fs;
use std::path;
use littletest;

use adapter::{Adapter};

pub struct RunOptions<'o> {
    pub ignore_todo: bool,
    pub engine: Box<Adapter + 'o>
}

pub struct TestCase<'o> {
    pub input_path: path::PathBuf,
    pub expected_path: path::PathBuf,
    pub opts: &'o RunOptions<'o>
}

impl<'o> TestCase<'o> {
    pub fn is_todo(&self) -> bool {
        match self.input_path.to_str() {
            Some(x) => x.contains("todo"),
            None => false
        }
    }
}

fn clean_output(css: &str) -> String {
    let despaced = regex!(r"\s+").replace_all(css, " ");
    let destarred = regex!(r" *\{").replace_all(despaced.as_ref(), " {\n");
    let newlined = regex!(r"([;,]) *").replace_all(destarred.as_ref(), "$1\n");
    let destarred2 = regex!(r" *\} *").replace_all(newlined.as_ref(), " }\n");
    let trim: &[_] = &[' ', '\t', '\n', '\r'];
    destarred2.trim_matches(trim).to_string()
}

impl<'a> littletest::Runnable for TestCase<'a> {
    fn run(&self) -> littletest::TestResult {
        use littletest::{TestResult};
        use std::io::Read;

        if self.opts.ignore_todo && self.is_todo() {
            return TestResult::Skipped
        }

        let result = self.opts.engine.compile(&self.input_path);

        if result.is_err() {
            return TestResult::Fail
        }
        let output = clean_output(result.unwrap().as_ref());

        let expected_display = self.expected_path.display();
        let mut expected_buffer = String::new();
        let expected = match fs::File::open(&self.expected_path) {
            Err(why) => panic!("couldn't open {}: {}", expected_display, why),
            Ok(mut file) => match file.read_to_string(&mut expected_buffer) {
                Err(why) => panic!("couldn't read {}: {}", expected_display, why),
                Ok(_) => clean_output(expected_buffer.as_ref())
            }
        };

        if output != expected {
            return TestResult::Fail
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
