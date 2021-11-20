use crate::entry::Entry;

pub struct NotificationTestEntry {}

impl Entry for NotificationTestEntry {
    fn name() -> String {
        "notification test".to_string()
    }

    fn action() {
        Self::execute_command("notify-send", &["TEST", "test"])
    }
}
