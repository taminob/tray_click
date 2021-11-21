use crate::entry::Entry;
use gtk;

#[derive(Default)]
pub struct ExitEntry {}

impl Entry for ExitEntry {
    fn name(&self) -> String {
        "exit".to_string()
    }

    fn action(&self) {
        gtk::main_quit()
    }
}
