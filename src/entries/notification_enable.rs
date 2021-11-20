use crate::entry::Entry;

pub struct NotificationEnableEntry {}

impl Entry for NotificationEnableEntry {
    fn name() -> String {
        "enable notifications".to_string()
    }

    fn action() {
        Self::execute_command("makoctl", &["set-mode", "default"])
    }
}
