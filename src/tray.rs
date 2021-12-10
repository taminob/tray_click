use crate::entries;
use crate::entry::Entry;
use std::marker::{Send, Sync};
use toml;
use tray_item::TrayItem;

pub fn create_tray(app_name: &str, icon: &str, custom_items: Vec<&str>) {
    let mut tray = TrayItem::new(app_name, icon).expect("failed to create tray");
    tray.add_label(app_name).expect("failed to add tray label");
    for item in custom_items.chunks(3) {
        add_custom_item_to_tray(
            &mut tray,
            &("display = \"".to_owned()
                + item[2]
                + "\"\ncommand = \""
                + item[0]
                + "\"\nargs = \""
                + item[1]
                + "\"\nenabled = true"),
        );
    }
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::CustomEntry>(&mut tray);
    add_custom_item_to_tray(
        &mut tray,
        r#"
        display = "echo other"
        enabled = true
        command = "echo"
        args = "this is an arg""#,
    );
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);
}

fn add_item_to_tray<EntryType: Entry + Default + Sync + Send + 'static>(tray: &mut TrayItem) {
    let new_item = EntryType::default();
    let item_name = new_item.name();
    tray.add_menu_item(item_name.as_str(), move || new_item.action())
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", item_name.as_str()));
}

fn add_custom_item_to_tray(tray: &mut TrayItem, custom_item: &str) {
    let new_item: entries::CustomEntry =
        toml::from_str(custom_item).expect("invalid config for custom entry");
    let item_name = new_item.name();
    tray.add_menu_item(item_name.as_str(), move || new_item.action())
        .unwrap_or_else(|_| panic!("failed to add custom tray item: {}", custom_item));
}
