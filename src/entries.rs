macro_rules! declare_entry {
    ($m:ident) => {
        pub mod $m;
        pub use $m::*;
    }
}

declare_entry!(custom);
declare_entry!(echo);
declare_entry!(notification_test);
declare_entry!(notification_enable);
declare_entry!(notification_disable);
declare_entry!(exit);
