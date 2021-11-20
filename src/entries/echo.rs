use crate::entry::Entry;

pub struct EchoEntry {}

impl Entry for EchoEntry {
    fn name() -> String {
        "echo test".to_string()
    }

    fn action() {
        Self::execute_command("echo", &["hello world"])
    }
}
