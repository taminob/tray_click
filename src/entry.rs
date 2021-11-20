use std::process::{Command, Output};

pub trait Entry {
    fn name() -> String;
    fn action();

    fn execute_command(program: &str, args: &[&str]) {
        helper::print_command_output(Self::output_command(program, args));
    }

    fn output_command(program: &str, args: &[&str]) -> Output {
        let mut command = Command::new(program);
        for arg in args {
            command.arg(arg);
        }
        command.output()
               .expect(helper::command_exec_error_msg(program).as_str())
    }
}

mod helper {
    use super::Output;

    pub fn command_exec_error_msg(program_name: &str) -> String {
        format!("{} {}", "unable to execute", program_name)
    }

    pub fn print_command_output(output: Output) {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
