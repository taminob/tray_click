use crate::entry::Entry;

#[derive(Default)]
pub struct CustomEntry {
    entry_name: String,
    command: String,
    args: Vec<&'static str>,
}

impl Entry for CustomEntry {
    fn name(&self) -> String {
        self.entry_name.clone()
    }

    fn action(&self) {
        if !self.command.is_empty() {
            Self::execute_command(self.command.as_str(), self.args.as_slice())
        }
    }
}
