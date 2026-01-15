# stsr

Minimal single-threaded async status updater that writes your system summary into the `X11` root window name. Pair it with a bar (e.g. `dwm`) and keep your status lean, fast, and configurable in Rust. External `Command`/`Shell` sources are executed in separate processes.

## Features
- Per-source refresh intervals (CPU, RAM, battery, commands, time).
- Centralized error handling: failures log to `stderr` and show `err` on the bar.
- Simple percentage helpers with saturating math for stable output.
- Configurable output format strings (e.g. ` {}`).
- Spawns external `Command`/`Shell` sources as separate processes, while orchestration runs on a single async runtime thread.

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
- `interval`: seconds between updates for that source; missed ticks are skipped for long runs.
- `timeout`: seconds before a `Command`/`Shell` run is considered hung and returns `err`.
- `Source` options: `Cpu`, `Ram`, `Battery`, `Command`, `Shell Script`, `DateTime`.

Example snippet (from `src/config.rs`):
```rust
Status {
    source: Source::Command {
        cmd: "curl",
        args: &["wttr.in?format=%c%t"],
        timeout: 120,
    },
    format: "",
    default: "...",
    interval: 600,
}
```
check out `src/config.rs` for more examples.

## Timing and timeouts
Each status runs serially: a new run does not start until the previous one finishes.
If a run exceeds its `interval`, missed ticks are skipped and the next run starts immediately after completion.
`Command` and `Shell` use per-source `timeout` (seconds). On timeout, the status logs an error and shows `err`.

## Sources and expectations
- CPU: reads `/proc/stat`, reports total CPU usage percent.
- RAM: uses `MemTotal` and `MemAvailable` from `/proc/meminfo`.
- Battery: reads capacity from `/sys/class/power_supply/<NAME>/capacity`.
- Command: runs the given program with `args` in a separate process; uses per-source `timeout` (seconds).
- Shell: runs the given script via `sh -c` in a separate process; uses per-source `timeout` (seconds).
- Date/time: formats with the configured `chrono_tz` timezone (adjust in `config.rs`).
