use crate::entries;
use crate::entry;
use std::marker::{Send, Sync};
use tray_item::TrayItem;

pub fn create_tray(app_name: &str, icon: &str) {
    let mut tray = TrayItem::new(app_name, icon).expect("failed to create tray");
    tray.add_label(app_name).expect("failed to add tray label");
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::CustomEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);
}

fn add_item_to_tray<EntryType: entry::Entry + Default + Sync + Send + 'static>(
    tray: &mut TrayItem,
) {
    let new_item = EntryType::default();
    let item_name = new_item.name();
    tray.add_menu_item(item_name.as_str(), move || new_item.action())
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", item_name.as_str()));
}
