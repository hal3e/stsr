// when releasing use:
//    CHRONO_TZ_TIMEZONE_FILTER="(Europe/Vienna)" cargo build --release

mod config;
mod status;

#[tokio::main]
async fn main() {
    let statuses = config::statuses();

    status::Bar::new(statuses)
        .with_replace_marker("{}")
        .with_separator(" ")
        .run()
        .await;
}
