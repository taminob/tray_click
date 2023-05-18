use crate::entries;
use crate::entry;
use std::marker::{Send, Sync};
use std::str::FromStr;
use tray_item::TrayItem;

pub fn create_tray(
    app_name: &str,
    icon: &str,
    enabled_items: Vec<&str>,
    custom_items: Vec<&str>,
    config_files: Vec<&str>,
) {
    let mut tray = TrayItem::new(app_name, icon).expect("failed to create tray");
    tray.add_label(app_name).expect("failed to add tray label");
    for item in enabled_items {
        add_item_to_tray(&mut tray, item);
    }
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
    for file in config_files {
        add_custom_item_to_tray(
            &mut tray,
            std::fs::read_to_string(file)
                .unwrap_or_else(|x| {
                    println!("failed to read: {}", x);
                    String::new()
                })
                .as_str(),
        );
    }
}

fn add_item_to_tray(tray: &mut TrayItem, item: &str) {
    match entries::DeclaredEntry::from_str(item).expect("invalid declared entry name") {
        entries::DeclaredEntry::Echo(x) => add_entry_to_tray(tray, x),
        entries::DeclaredEntry::NotificationTest(x) => add_entry_to_tray(tray, x),
        entries::DeclaredEntry::NotificationEnable(x) => add_entry_to_tray(tray, x),
        entries::DeclaredEntry::NotificationDisable(x) => add_entry_to_tray(tray, x),
        entries::DeclaredEntry::Exit(x) => add_entry_to_tray(tray, x),
    }
}

fn add_custom_item_to_tray(tray: &mut TrayItem, custom_item: &str) {
    if custom_item.is_empty() {
        return;
    }
    match toml::from_str::<entries::CustomEntry>(custom_item) {
        Ok(new_item) => add_entry_to_tray(tray, new_item),
        Err(error) => println!("invalid config for custom entry: {}", error.to_string()),
    }
}

fn add_entry_to_tray<EntryType: 'static + entry::Entry + Send + Sync>(
    tray: &mut TrayItem,
    entry: EntryType,
) {
    let entry_name = entry.name();
    tray.add_menu_item(entry_name.as_str(), move || entry.action())
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", entry_name));
}
