use gtk;
use std::convert::TryFrom;
use std::process::{Command, Output};
use tray_item::TrayItem;

const APPLICATION_NAME: &str = "clickerrr";
const APPLICATION_ICON: &str = "arrow-down-double";

enum NotificationMode {
    Default,
    DoNotDisturb,
}

impl NotificationMode {
    fn value(&self) -> &'static str {
        match *self {
            NotificationMode::Default => "default",
            NotificationMode::DoNotDisturb => "dnd",
        }
    }
}

impl TryFrom<&str> for NotificationMode {
    type Error = ();

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            _ if v == Self::Default.value() => Ok(Self::Default),
            _ if v == Self::DoNotDisturb.value() => Ok(Self::DoNotDisturb),
            _ => Err(()),
        }
    }
}

fn command_exec_error_msg(program_name: &str) -> String {
    format!("{} {}", "unable to execute", program_name)
}

fn set_notifications_mode(new_mode: NotificationMode) -> Output {
    const PROGRAM: &'static str = "/usr/bin/makoctl";
    Command::new(PROGRAM)
        .arg("set-mode")
        .arg(new_mode.value())
        .output()
        .expect(command_exec_error_msg(PROGRAM).as_str())
}

fn send_test_notification(test_string: &str) -> Output {
    const PROGRAM: &'static str = "/usr/bin/notify-send";
    Command::new(PROGRAM)
        .arg(test_string)
        .output()
        .expect(command_exec_error_msg(PROGRAM).as_str())
}

fn print_command_output(output: Output) {
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn main() {
    gtk::init().expect("failed to init gtk");

    let mut tray = TrayItem::new(APPLICATION_NAME, APPLICATION_ICON).expect("failed to create tray");
    tray.add_label(APPLICATION_NAME).expect("failed to add tray label");
    tray.add_menu_item("echo test", || {
        let output = Command::new("echo")
            .arg("hello world!")
            .output()
            .expect("unable to execute echo");
        print_command_output(output);
    })
    .expect("failed to add tray item: echo");

    tray.add_menu_item("notification test", || {
        let output = send_test_notification("test");
        print_command_output(output);
    })
    .expect("failed to add tray item: notification test");

    tray.add_menu_item("Disable notifications", || {
        let output = set_notifications_mode(NotificationMode::DoNotDisturb);
        print_command_output(output);
    })
    .expect("failed to add tray item: disable notifications");

    tray.add_menu_item("Enable notifications", || {
        let output = set_notifications_mode(NotificationMode::Default);
        print_command_output(output);
    })
    .expect("failed to add tray item: enable notifications");

    tray.add_menu_item("exit", || {
        gtk::main_quit();
    })
    .expect("failed to add tray item: exit");

    gtk::main();
}
