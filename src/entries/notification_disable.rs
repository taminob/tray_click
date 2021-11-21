use crate::entry::Entry;

#[derive(Default)]
pub struct NotificationDisableEntry {}

impl Entry for NotificationDisableEntry {
    fn name(&self) -> String {
        "disable notifications".to_string()
    }

    fn action(&self) {
        Self::execute_command("makoctl", &["set-mode", "do-not-disturb"])
    }
}
