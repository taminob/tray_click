//use gtk::{init, main_quit};
use tray_item::TrayItem;
mod entry;
mod entries;

const APPLICATION_NAME: &str = "clickerrr";
const APPLICATION_ICON: &str = "arrow-down-double";

fn add_item_to_tray<EntryType: 'static>(tray: &mut TrayItem) where EntryType: entry::Entry {
    tray.add_menu_item(EntryType::name().as_str(), EntryType::action).expect(format!("failed to add tray item: {}", EntryType::name().as_str()).as_str());
}

fn main() {
    gtk::init().expect("failed to init gtk");

    let mut tray = TrayItem::new(APPLICATION_NAME, APPLICATION_ICON).expect("failed to create tray");
    tray.add_label(APPLICATION_NAME).expect("failed to add tray label");
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);

    gtk::main();
}
