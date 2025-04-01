use crate::status::{sources::Source, Status};

pub fn statuses() -> Vec<Status> {
    vec![
        Status {
            source: Source::cpu(),
            format: " {}%".to_string(),
            default: "n/a".to_string(),
            sec: 1,
        },
        Status {
            source: Source::Ram,
            format: " {}%".to_string(),
            default: "n/a".to_string(),
            sec: 2,
        },
        Status {
            source: Source::Battery {
                name: "BAT0".to_string(),
            },
            format: " {}%".to_string(),
            default: "n/a".to_string(),
            sec: 60,
        },
        Status {
            source: Source::DateTime {
                format: "%d/%m %a".to_string(),
            },
            format: " {}".to_string(),
            default: "n/a".to_string(),
            sec: 1,
        },
        Status {
            source: Source::DateTime {
                format: "%H:%M".to_string(),
            },
            format: " {}".to_string(),
            default: "n/a".to_string(),
            sec: 1,
        },
    ]
}
