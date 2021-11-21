use crate::entry::Entry;

#[derive(Default)]
pub struct NotificationEnableEntry {}

impl Entry for NotificationEnableEntry {
    fn name(&self) -> String {
        "enable notifications".to_string()
    }

    fn action(&self) {
        Self::execute_command("makoctl", &["set-mode", "default"])
    }
}
