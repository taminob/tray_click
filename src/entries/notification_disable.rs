use crate::entry::Entry;

pub struct NotificationDisableEntry {}

impl Entry for NotificationDisableEntry {
    fn name() -> String {
        "disable notifications".to_string()
    }

    fn action() {
        Self::execute_command("makoctl", &["set-mode", "do-not-disturb"])
    }
}
