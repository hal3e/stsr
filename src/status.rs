use std::{cell::RefCell, fmt};

use futures::future::join_all;
use tokio::{
    signal,
    time::{Duration, MissedTickBehavior},
};

use crate::x11::X11rb;

#[derive(Debug, Clone)]
pub struct Error(pub String);

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub mod sources;
mod utils;

#[derive(Debug)]
pub struct Status {
    pub source: sources::Source,
    pub format: &'static str,
    pub default: &'static str,
    // Run interval in seconds
    pub interval: u64,
}

impl Status {
    pub async fn run(&mut self, shared_output: &RefCell<String>, replace_marker: &str) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.interval));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            let output = match self.source.output().await {
                Ok(output) if output.is_empty() => self.default.to_string(),
                Ok(output) => output,
                Err(err) => {
                    eprintln!("{}: {err}", self.source.label());
                    "err".to_string()
                }
            };

            if self.format.is_empty() {
                *shared_output.borrow_mut() = output;
            } else {
                *shared_output.borrow_mut() = self.format.replace(replace_marker, &output);
            }
        }
    }
}

#[derive(Debug)]
pub struct Bar {
    statuses: Vec<Status>,
    x11rb: X11rb,
    replace_marker: String,
    separator: String,
    write_interval: Duration,
}

impl Bar {
    pub const fn new(statuses: Vec<Status>, x11rb: X11rb) -> Self {
        Self {
            statuses,
            x11rb,
            replace_marker: String::new(),
            separator: String::new(),
            write_interval: Duration::from_millis(500),
        }
    }

    pub fn with_replace_marker(mut self, marker: &str) -> Self {
        self.replace_marker = marker.to_string();
        self
    }

    pub fn with_separator(mut self, separator: &str) -> Self {
        self.separator = separator.to_string();
        self
    }

    pub const fn with_write_interval(mut self, write_interval: Duration) -> Self {
        self.write_interval = write_interval;
        self
    }

    async fn write_output(
        write_interval: Duration,
        outputs: &[RefCell<String>],
        separator: &str,
        x11rb: &X11rb,
    ) {
        let mut interval = tokio::time::interval(write_interval);
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut last_push = String::new();
        loop {
            interval.tick().await;

            let mut accumulated_output = String::new();
            let mut outputs_iter = outputs.iter().peekable();
            while let Some(output) = outputs_iter.next() {
                accumulated_output.push_str(&output.borrow());

                if outputs_iter.peek().is_some() {
                    accumulated_output.push_str(separator);
                }
            }

            if accumulated_output != last_push {
                if let Err(err) = x11rb.set_root_win_name(&accumulated_output) {
                    eprint!("error writing root window name: {err}");
                } else {
                    last_push = accumulated_output;
                }
            }
        }
    }

    async fn run_inner(&mut self) {
        let shared_outputs: Vec<RefCell<String>> = self
            .statuses
            .iter()
            .map(|c| RefCell::new(c.default.to_string()))
            .collect();

        let run_futures = join_all(
            self.statuses
                .iter_mut()
                .zip(shared_outputs.iter())
                .map(|(status, output)| status.run(output, &self.replace_marker)),
        );
        let write_output_future = Self::write_output(
            self.write_interval,
            &shared_outputs,
            &self.separator,
            &self.x11rb,
        );

        tokio::join!(run_futures, write_output_future);
    }

    pub async fn run(&mut self) {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to instantiate unix SIGTERM handler");

        tokio::select! {
            () = self.run_inner() => {
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
}
