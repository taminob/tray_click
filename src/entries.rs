use std::str::FromStr;

macro_rules! declare_entry {
    ($m:ident) => {
        pub mod $m;
        pub use $m::*;
    };
}

declare_entry!(custom);
declare_entry!(echo);
declare_entry!(notification_test);
declare_entry!(notification_enable);
declare_entry!(notification_disable);
declare_entry!(exit);

pub enum DeclaredEntry {
    Echo(EchoEntry),
    NotificationTest(NotificationTestEntry),
    NotificationEnable(NotificationEnableEntry),
    NotificationDisable(NotificationDisableEntry),
    Exit(ExitEntry),
}

impl FromStr for DeclaredEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo(EchoEntry::default())),
            "notification_test" => Ok(Self::NotificationTest(NotificationTestEntry::default())),
            "notification_enable" => {
                Ok(Self::NotificationEnable(NotificationEnableEntry::default()))
            }
            "notification_disable" => Ok(Self::NotificationDisable(
                NotificationDisableEntry::default(),
            )),
            "exit" => Ok(Self::Exit(ExitEntry::default())),
            _ => Err(()),
        }
    }
}
