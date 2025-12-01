// when releasing use:
//    CHRONO_TZ_TIMEZONE_FILTER="(Europe/Vienna)" cargo build --release

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

    bar.run().await;
}
