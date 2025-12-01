use crate::status::{Status, sources::Source};

pub fn statuses() -> Vec<Status> {
    vec![
        Status {
            source: Source::cpu(),
            format: " {}%",
            default: " n/a%",
            interval: 1,
        },
        Status {
            source: Source::Ram,
            format: " {}%",
            default: " n/a%",
            interval: 2,
        },
        Status {
            source: Source::Battery { name: "BAT0" },
            format: " {}%",
            default: " 0%",
            interval: 60,
        },
        Status {
            source: Source::Command {
                cmd: "curl",
                args: &["wttr.in?format=%c%t"],
            },
            format: "",
            default: "n/a",
            interval: 600,
        },
        Status {
            source: Source::DateTime { format: "%d/%m %a" },
            format: " {}",
            default: " n/a",
            interval: 1,
        },
        Status {
            source: Source::DateTime { format: "%H:%M" },
            format: " {}",
            default: " n/a",
            interval: 1,
        },
    ]
}
