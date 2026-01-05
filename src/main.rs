use std::time::Duration;

mod config;
mod status;
mod x11;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let statuses = config::statuses();
    let x11rb = x11::X11rb::new(5).expect("can not connect to x11 server");

    let mut bar = status::Bar::new(statuses, x11rb)
        .with_replace_marker("{}")
        .with_separator(" ")
        .with_write_interval(Duration::from_secs(1))
        .with_write_to_stdout(false)
        .with_write_on_changes(true);

    bar.run().await;
}
