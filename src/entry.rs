use std::process::{Command, Output};

pub trait Entry {
    fn name(&self) -> String;
    fn action(&self);

    fn execute_command<I: IntoIterator<Item = S>, S: AsRef<str>>(&self, program: &str, args: I) {
        helper::print_command_output(self.output_command(program, args));
    }

    fn output_command<I: IntoIterator<Item = S>, S: AsRef<str>>(
        &self,
        program: &str,
        args: I,
    ) -> Output {
        let mut command = Command::new(program);
        for arg in args {
            command.arg(arg.as_ref());
        }
        command.output().unwrap_or_else(|_| {
            panic!("{}", helper::command_exec_error_msg(program));
        })
    }
}

mod helper {
    use super::Output;

    pub fn command_exec_error_msg(program_name: &str) -> String {
        format!("unable to execute: {}", program_name)
    }

    pub fn print_command_output(output: Output) {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
