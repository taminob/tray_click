use clap::{App, Arg};
use tray_item::TrayItem;
mod entries;
mod entry;

const DEFAULT_APPLICATION_NAME: &str = "tray_click";
const DEFAULT_APPLICATION_ICON: &str = "arrow-down-double";

fn add_item_to_tray<EntryType: 'static>(tray: &mut TrayItem)
where
    EntryType: entry::Entry,
{
    tray.add_menu_item(EntryType::name().as_str(), EntryType::action)
        .unwrap_or_else(|_| panic!("failed to add tray item: {}", EntryType::name().as_str()));
}

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("name")
                .help("Set the display name for the application")
                .short("n")
                .long("name")
                .takes_value(true)
                .value_name("STRING")
                .default_value(DEFAULT_APPLICATION_NAME),
        )
        .arg(
            Arg::with_name("icon")
                .help("Set the display icon for the application")
                .short("i")
                .long("icon")
                .takes_value(true)
                .value_name("STRING")
                .default_value(DEFAULT_APPLICATION_ICON),
        )
        .get_matches();

    gtk::init().expect("failed to init gtk");
    let app_name = args.value_of("name").unwrap();
    glib::set_application_name(app_name);

    let mut tray =
        TrayItem::new(app_name, args.value_of("icon").unwrap()).expect("failed to create tray");
    tray.add_label(app_name).expect("failed to add tray label");
    add_item_to_tray::<entries::EchoEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationTestEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationEnableEntry>(&mut tray);
    add_item_to_tray::<entries::NotificationDisableEntry>(&mut tray);
    add_item_to_tray::<entries::ExitEntry>(&mut tray);

    gtk::main();
}
