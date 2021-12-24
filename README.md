# tray_click ![CI Status](https://github.com/taminob/tray_click/workflows/CI/badge.svg?branch=main)

An application written in Rust which creates tray icon which provides a dropdown menu with custom or pre-defined actions.

```shell
$ tray_click --help
tray_click 0.1.0
Tamino Bauknecht <dev@tb6.eu>


USAGE:
    tray_click [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --command <COMMAND> <ARGS> <NAME>    Add command to tray
    -e, --enable <enabled>...                Enable predefined command [possible values: echo, notification_test,
                                             notification_enable, notification_disable, exit]
    -f, --file <FILE>...                     Specify config file path [default: ~/.config/tray_click/config.toml]
    -i, --icon <FILE>                        Set the display icon for the application [default:
                                             resources/tray_click.png]
    -n, --name <STRING>                      Set the display name for the application [default: tray_click]
```
