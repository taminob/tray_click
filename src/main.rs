use clap::{App, Arg, Values};
mod entries;
mod entry;
mod tray;

const DEFAULT_APPLICATION_NAME: &str = "tray_click";
const DEFAULT_APPLICATION_ICON: &str = "arrow-down-double";

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
        .arg(
            Arg::with_name("command")
                .help("Add command to tray")
                .short("c")
                .long("command")
                .takes_value(true)
                .number_of_values(3)
                .multiple(true)
                .value_names(&["COMMAND", "ARGS", "NAME"]),
        )
        .get_matches();

    gtk::init().expect("failed to init gtk");
    let app_name = args.value_of("name").unwrap();
    glib::set_application_name(app_name);

    tray::create_tray(
        app_name,
        args.value_of("icon").unwrap(),
        args.values_of("command")
            .unwrap_or_else(|| Values::default())
            .collect(),
    );

    gtk::main();
}
