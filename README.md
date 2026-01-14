# stsr

Minimal single threaded and async status updater that writes your system summary into the `X11` root window name. Pair it with a bar (e.g. `dwm`) and keep your status lean, fast, and configurable in Rust.

## Features
- Per-source refresh intervals (CPU, RAM, battery, commands, time).
- Centralized error handling: failures log to `stderr` and show `err` on the bar.
- Simple percentage helpers with saturating math for stable output.
- Configurable output format strings (e.g. ` {}`).

## Requirements
- Rust (edition 2024), Cargo, and an X11 session.
- The `x11rb` dependency talks to your X server; ensure `$DISPLAY` is set.
- Weather example uses `curl` and `wttr.in`; swap or remove if undesired.

## Getting Started
For a release build (with timezone filtering to shrink dependencies):
```sh
CHRONO_TZ_TIMEZONE_FILTER="(Europe/Vienna)" cargo build --release
```

## Configuration
Edit `src/config.rs` to change the displayed sources, order, and formatting.

- `format`: how the value is embedded (uses `{}` as the marker by default).
- `default`: raw placeholder shown until the first successful fetch (also passed through `format`).
- `interval`: seconds between updates for that source.
- `Source` options: `Cpu`, `Ram`, `Battery`, `Command`, `Shell Script`, `DateTime`.

Example snippet (from `src/config.rs`):
```rust
Status {
    source: Source::Command { cmd: "curl", args: &["wttr.in?format=%c%t"] },
    format: "",
    default: "n/a",
    interval: 600,
}
```
check out `src/config.rs` for more examples.

## Sources and expectations
- CPU: reads `/proc/stat`, reports total CPU usage percent.
- RAM: uses `MemTotal` and `MemAvailable` from `/proc/meminfo`.
- Battery: reads capacity from `/sys/class/power_supply/<NAME>/capacity`.
- Command: runs the given program with `args`; non-zero exit or invalid UTF-8 -> `err`.
- Shell: runs the given script via `sh -c`.
- Date/time: formats with the configured `chrono_tz` timezone (adjust in `config.rs`).
