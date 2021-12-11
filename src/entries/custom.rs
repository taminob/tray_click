use crate::entry::Entry;
use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
pub struct CustomEntry {
    display: String,
    command: String,
    args: String,
}

impl Entry for CustomEntry {
    fn name(&self) -> String {
        self.display.clone()
    }

    fn action(&self) {
        if !self.command.is_empty() {
            self.execute_command(self.command.as_str(), &[self.args.as_str()])
        }
    }
}
