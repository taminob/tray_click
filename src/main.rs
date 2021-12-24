use clap::{App, Arg};
use std::path::{Path, PathBuf};
mod entries;
mod entry;
mod tray;

const DEFAULT_APPLICATION_NAME: &str = "tray_click";
const DEFAULT_APPLICATION_ICON: &str = "/usr/share/tray_click/icon.png";
const DEFAULT_CONFIG_PATH: &str = "~/.config/tray_click/config.toml";

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
                .value_name("FILE")
                .default_value(DEFAULT_APPLICATION_ICON),
        )
        .arg(
            Arg::with_name("file")
                .help("Specify config file path")
                .short("f")
                .long("file")
                .takes_value(true)
                .multiple(true)
                .value_name("FILE")
                .default_value(DEFAULT_CONFIG_PATH),
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
        .arg(
            Arg::with_name("enabled")
                .help("Enable predefined command")
                .short("e")
                .long("enable")
                .takes_value(true)
                .multiple(true)
                .possible_values(&[
                    "echo",
                    "notification_test",
                    "notification_enable",
                    "notification_disable",
                    "exit",
                ]),
        )
        .get_matches();

    gtk::init().expect("failed to init gtk");
    let app_name = args.value_of("name").unwrap();
    glib::set_application_name(app_name);

    let tray_icon = Path::new(args.value_of("icon").unwrap()).canonicalize();

    tray::create_tray(
        app_name,
        tray_icon
            .unwrap_or_else(|x| {
                println!("unable to load icon: {}", x);
                PathBuf::from(DEFAULT_APPLICATION_ICON)
            })
            .to_str()
            .expect("icon path is not utf-8"),
        args.values_of("enabled").unwrap_or_default().collect(),
        args.values_of("command").unwrap_or_default().collect(),
    );

    gtk::main();
}
