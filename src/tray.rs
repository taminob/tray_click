use crate::entries;
use crate::entry;
use tray_item::TrayItem;

pub fn create_tray(app_name: &str, icon: &str) {
    let mut tray = TrayItem::new(app_name, icon).expect("failed to create tray");
    tray.add_label(app_name).expect("failed to add tray label");
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);
}

fn add_item_to_tray<EntryType: 'static>(tray: &mut TrayItem)
where
    EntryType: entry::Entry,
{
    tray.add_menu_item(EntryType::name().as_str(), EntryType::action)
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", EntryType::name().as_str()));
}
