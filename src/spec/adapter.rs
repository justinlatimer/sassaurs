use std::path;

pub trait Adapter : Sync {
    fn description(&self) -> &str;
    fn version(&self) -> String;
    fn compile(&self, input_path: &path::PathBuf) -> Option<String>;
}

pub struct ExecutableAdapter<'c> {
    command: &'c str
}

impl<'c> ExecutableAdapter<'c> {
    pub fn new(command: &'c str) -> ExecutableAdapter {
        ExecutableAdapter {
            command: command
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

impl<'c> Adapter for ExecutableAdapter<'c> {
    fn description(&self) -> &str {
        self.command
    }

    fn version(&self) -> String {
        let executable = self.command.split(' ').next().unwrap();
        match exec(executable, &["-v"]) {
            Some(output) => output,
            None => match exec(executable, &["-V"]) {
                Some(output) => output,
                None => panic!("Could not get version of {}", executable)
            }
        }
    }

    fn compile(&self, input_path: &path::PathBuf) -> Option<String> {
        let command_input: &str = self.command.as_ref();
        let mut parts = command_input.split(' ');
        let command = parts.next().unwrap();
        let mut rest: Vec<&str> = parts.collect();
        rest.push(input_path.to_str().unwrap());

        exec(command, rest.as_ref())
    }
}
