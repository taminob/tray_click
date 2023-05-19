# tray_click ![CI Status](https://github.com/taminob/tray_click/workflows/CI/badge.svg?branch=main)

An application written in Rust which creates a tray icon which provides
a dropdown menu with custom or pre-defined actions.

# Usage

```shell
$ tray_click --help
tray_click 0.1.0
Tamino Bauknecht <dev@tb6.eu>


USAGE:
    tray_click [OPTIONS] [--] [command]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --enable <enabled>...    Enable predefined command [possible values: echo, notification_test,
                                 notification_enable, notification_disable, exit]
    -f, --file <FILE>...         Specify config file path [default: ~/.config/tray_click/config.toml]
    -i, --icon <FILE>            Set the display icon for the application [default: /usr/share/tray_click/icon.png]
    -n, --name <STRING>          Set the display name for the application [default: tray_click]

ARGS:
    <command>...    Add one additional command to tray (<command>... consists of <name> <executable> [<args...>])
```

Example for:

- enabling an in-built entry,
- adding entries from configuration files and
- specifying a custom command

```shell
$ tray_click --enable notification_test \
    --file examples/calculator.toml examples/echo_a.toml \
    -- "Terminal" "/usr/bin/alacritty" "--working-directory" "/"
```
