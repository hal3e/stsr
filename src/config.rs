use std::time::Duration;

use chrono_tz::Europe::Vienna;

use crate::{
    error::{Error, Result},
    status::{Status, sources::Source},
};

/// Maximum time to wait for a command to complete before timing out
pub const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);

fn status_definitions() -> Vec<Status> {
    vec![
        Status {
            source: Source::cpu(),
            format: " {}%",
            default: "0",
            interval: 1,
        },
        Status {
            source: Source::Ram,
            format: " {}%",
            default: "0",
            interval: 2,
        },
        Status {
            source: Source::Battery { name: "BAT0" },
            format: " {}%",
            default: "0",
            interval: 60,
        },
        Status {
            source: Source::Shell {
                script: r#"
                    set -e
                    host="$(hostname)"
                    disk="$(df -h / | awk 'NR==2 {print $5}')"
                    printf '%s %s' "$host" "$disk"
                "#,
            },
            format: " {}",
            default: "...",
            interval: 30,
        },
        Status {
            source: Source::Command {
                cmd: "curl",
                args: &["wttr.in?format=%c%t"],
            },
            format: "",
            default: "...",
            interval: 600,
        },
        Status {
            source: Source::DateTime {
                format: "%d/%m %a",
                timezone: Vienna,
            },
            format: " {}",
            default: "...",
            interval: 1,
        },
        Status {
            source: Source::DateTime {
                format: "%H:%M",
                timezone: Vienna,
            },
            format: " {}",
            default: "...",
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

    Ok(statuses)
}
