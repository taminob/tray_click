use crate::entry::Entry;

#[derive(Default)]
pub struct NotificationTestEntry {}

impl Entry for NotificationTestEntry {
    fn name(&self) -> String {
        "notification test".to_string()
    }

    fn action(&self) {
        self.execute_command("notify-send", &["TEST", "test"])
    }
}
