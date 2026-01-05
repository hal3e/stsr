use std::time::Duration;

use crate::status::{Status, sources::Source};

/// Maximum time to wait for a command to complete before timing out
pub const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);

pub fn statuses() -> Vec<Status> {
    vec![
        Status {
            source: Source::cpu(),
            format: " {}%",
            default: " .%",
            interval: 1,
        },
        Status {
            source: Source::Ram,
            format: " {}%",
            default: " .%",
            interval: 2,
        },
        Status {
            source: Source::Battery { name: "BAT0" },
            format: " {}%",
            default: " .%",
            interval: 60,
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
            source: Source::DateTime { format: "%d/%m %a" },
            format: " {}",
            default: " ...",
            interval: 1,
        },
        Status {
            source: Source::DateTime { format: "%H:%M" },
            format: " {}",
            default: " ...",
            interval: 1,
        },
    ]
}
