use crate::entry::Entry;

#[derive(Default)]
pub struct EchoEntry {}

impl Entry for EchoEntry {
    fn name(&self) -> String {
        "echo test".to_string()
    }

    fn action(&self) {
        self.execute_command("echo", &["test"])
    }
}
