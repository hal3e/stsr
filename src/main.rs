// when releasing use:
//    CHRONO_TZ_TIMEZONE_FILTER="(Europe/Vienna)" cargo build --release

use tokio::signal;

mod config;
mod status;
mod x11;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let statuses = config::statuses();
    let x11rb = x11::X11rb::new().expect("can not connect to x11 server");

    let mut bar = status::Bar::new(statuses, x11rb)
        .with_replace_marker("{}")
        .with_separator(" ");

    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("failed to instantiate unix SIGTERM handler");
    tokio::select! {
        _ = bar.run() => {
            eprintln!("status bar exited unexpectedly");
        }
        _ = signal::ctrl_c() => {
            eprintln!("received SIGINT (Ctrl+C), shutting down gracefully");
        }
        _ = sigterm.recv() => {
            eprintln!("received SIGTERM, shutting down gracefully");
        }
    }
}
