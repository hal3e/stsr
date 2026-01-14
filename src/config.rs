use chrono_tz::Europe::Vienna;

use crate::{
    error::{Error, Result},
    status::{Status, sources::Source},
};

fn status_definitions() -> Vec<Status> {
    vec![
        Status {
            source: Source::cpu(),
            format: " ^b ^n{}%",
            default: "0",
            interval: 1,
        },
        Status {
            source: Source::Ram,
            format: " ^t ^n{}%",
            default: "0",
            interval: 2,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    current_sink=$(pactl get-default-sink)
                    vol=$(pactl get-sink-volume "$current_sink" | tr -s " " | cut -d " " -f5)

                    icon=
                    if [ "$current_sink" = "bluez_output.80_99_E7_4A_8A_9D.1" ]; then
                        icon=
                    elif [ "$current_sink" = "bluez_output.AC_12_2F_5C_91_65.1" ]; then
                        icon=
                    fi

                    if [ "$(pactl get-sink-mute "$current_sink")" = "Mute: no" ]; then
                        echo " ^p$icon ^n$vol"
                    else
                        echo " ^m$icon ^m$vol"
                    fi
                "#,
                timeout: 1,
            },
            format: "",
            default: " ^p ^n0%",
            interval: 1,
        },
        Status {
            source: Source::Battery { name: "BAT0" },
            format: " ^y ^n{}%",
            default: "0",
            interval: 60,
        },
        Status {
            source: Source::Shell {
                script: "xbps-install -Mun | wc -l",
                timeout: 120,
            },
            format: " ^r ^n{}",
            default: "0",
            interval: 1800,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    set -e
                    location=Sarajevo
                    curl_res=$(curl -fsS wttr.in/${location}?format="%c+%t")
                    echo "$curl_res" | sed 's/[+C]//g' | sed 's/  /^n /g'
                "#,
                timeout: 120,
            },
            format: " ^g{}",
            default: ".°",
            interval: 600,
        },
        Status {
            source: Source::DateTime {
                format: "%d/%m %a",
                timezone: Vienna,
            },
            format: " ^b ^n{}",
            default: "dd/mm Day",
            interval: 1,
        },
        Status {
            source: Source::Command {
                cmd: "vaktija",
                args: &["sarajevo"],
                timeout: 1,
            },
            format: " ^t ^n{}",
            default: "hh:mm",
            interval: 1,
        },
        Status {
            source: Source::DateTime {
                format: "%H:%M",
                timezone: Vienna,
            },
            format: " ^g ^n{}",
            default: "hh:mm",
            interval: 1,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    set -e
                    wg show || echo " ^b"
                "#,
                timeout: 1,
            },
            format: "",
            default: "",
            interval: 1,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    set -e
                    is_bluetooth_on=$(bluetoothctl show | rg 'Powered: (.*)' -r '$1' | tr -d '\t')
                    test "$is_bluetooth_on" = "yes" && echo " ^b" || exit 0
                "#,
                timeout: 1,
            },
            format: "",
            default: "",
            interval: 1,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    interfaces=$(/bin/ls /sys/class/net)
                    wifi_interfaces=$(echo "$interfaces" | rg '^wl')
                    eth_interfaces=$(echo "$interfaces" | rg '^en')

                    for iface in $wifi_interfaces; do
                        if [ "$(cat /sys/class/net/"$iface"/operstate)" = "up" ]; then
                            echo " ^b"
                            exit 0
                        fi
                    done

                    for iface in $eth_interfaces; do
                        if [ "$(cat /sys/class/net/"$iface"/operstate)" = "up" ]; then
                            echo " ^b"
                            exit 0
                        fi
                    done
                "#,
                timeout: 1,
            },
            format: "",
            default: "",
            interval: 1,
        },
    ]
}

pub fn statuses() -> Result<Vec<Status>> {
    let statuses = status_definitions();

    if let Some(status) = statuses.iter().find(|status| status.interval == 0) {
        return Err(Error::config(format!(
            "status `interval` cannot be `0`: {status:?}"
        )));
    }

    if let Some(status) = statuses.iter().find(|status| {
        matches!(
            &status.source,
            Source::Command { timeout: 0, .. } | Source::Shell { timeout: 0, .. }
        )
    }) {
        return Err(Error::config(format!(
            "status `timeout` cannot be `0`: {status:?}"
        )));
    }

    Ok(statuses)
}
