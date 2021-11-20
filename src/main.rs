use tray_item::TrayItem;
mod entries;
mod entry;

const APPLICATION_NAME: &str = "tray_click";
const APPLICATION_ICON: &str = "arrow-down-double";

fn add_item_to_tray<EntryType: 'static>(tray: &mut TrayItem)
where
    EntryType: entry::Entry,
{
    tray.add_menu_item(EntryType::name().as_str(), EntryType::action)
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", EntryType::name().as_str()));
}

fn main() {
    gtk::init().expect("failed to init gtk");

    let mut tray =
        TrayItem::new(APPLICATION_NAME, APPLICATION_ICON).expect("failed to create tray");
    tray.add_label(APPLICATION_NAME)
        .expect("failed to add tray label");
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);

    gtk::main();
}
