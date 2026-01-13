use std::cell::RefCell;

use futures::future::join_all;
use tokio::{
    signal,
    time::{Duration, MissedTickBehavior},
};

use crate::{
    error::{Error, Result},
    x11::X11rb,
};

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
    fn format_value(&self, value: &str, replace_marker: &str) -> String {
        if self.format.is_empty() {
            value.to_string()
        } else {
            self.format.replace(replace_marker, value)
        }
    }

    fn default_output(&self, replace_marker: &str) -> String {
        self.format_value(self.default, replace_marker)
    }

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

            *shared_output.borrow_mut() = self.format_value(&output, replace_marker);
        }
    }
}

#[derive(Debug)]
pub struct Bar {
    statuses: Vec<Status>,
    x11rb: X11rb,
    /// Default replace marker is `{}`
    replace_marker: String,
    separator: String,
    write_interval: Duration,
    write_to_stdout: bool,
    /// Write output only when the content has changed
    write_on_changes: bool,
}

impl Bar {
    pub fn new(statuses: Vec<Status>, x11rb: X11rb) -> Self {
        Self {
            statuses,
            x11rb,
            replace_marker: String::from("{}"),
            separator: String::new(),
            write_interval: Duration::from_millis(500),
            write_to_stdout: true,
            write_on_changes: false,
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

    pub fn with_write_to_stdout(mut self, stdout: bool) -> Self {
        self.write_to_stdout = stdout;
        self
    }

    pub fn with_write_on_changes(mut self, write_on_changes: bool) -> Self {
        self.write_on_changes = write_on_changes;
        self
    }

    async fn write_output(
        write_interval: Duration,
        outputs: &[RefCell<String>],
        separator: &str,
        x11rb: &mut X11rb,
        write_to_stdout: bool,
        write_on_changes: bool,
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

            if !write_on_changes || accumulated_output != last_push {
                // X11rb handles reconnection internally; retry on next tick if it fails
                let write_ok = x11rb.set_root_win_name(&accumulated_output).is_ok();

                if write_to_stdout {
                    println!("{accumulated_output}");
                }

                if write_ok {
                    last_push = accumulated_output;
                }
            }
        }
    }

    async fn run_inner(&mut self) {
        let shared_outputs: Vec<RefCell<String>> = self
            .statuses
            .iter()
            .map(|c| RefCell::new(c.default_output(&self.replace_marker)))
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
            &mut self.x11rb,
            self.write_to_stdout,
            self.write_on_changes,
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
