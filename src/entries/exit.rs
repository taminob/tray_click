use crate::entry::Entry;
use gtk;

pub struct ExitEntry {}

impl Entry for ExitEntry {
    fn name() -> String {
        "exit".to_string()
    }

    fn action() {
        gtk::main_quit()
    }
}
