use std::cell::RefCell;

use futures::future::join_all;
use tokio::time::Duration;

use crate::x11::X11rb;

pub mod sources;
mod utils;

pub struct Bar {
    statuses: Vec<Status>,
    x11rb: X11rb,
    replace_marker: String,
    separator: String,
}

impl Bar {
    pub fn new(statuses: Vec<Status>, x11rb: X11rb) -> Self {
        Self {
            statuses,
            x11rb,
            replace_marker: String::new(),
            separator: String::new(),
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

    async fn write_output(outputs: &[RefCell<String>], separator: &str, x11rb: &X11rb) {
        loop {
            let mut accumulated_output = String::new();
            for output in outputs {
                accumulated_output.push_str(&output.borrow());
                accumulated_output.push_str(separator);
            }

            println!("{accumulated_output}");

            if let Err(err) = x11rb.set_root_win_name(&accumulated_output) {
                eprint!("error writing root window name: {err}");
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn run(&mut self) {
        let shared_outputs: Vec<RefCell<String>> = self
            .statuses
            .iter()
            .map(|c| RefCell::new(c.default.clone()))
            .collect();

        let run_futures = join_all(
            self.statuses
                .iter_mut()
                .zip(shared_outputs.iter())
                .map(|(status, output)| status.run(output, &self.replace_marker)),
        );

        tokio::join!(
            run_futures,
            Self::write_output(&shared_outputs, &self.separator, &self.x11rb)
        );
    }
}

pub struct Status {
    pub source: sources::Source,
    pub format: String,
    pub default: String,
    pub sec: u64,
}

impl Status {
    pub async fn run(&mut self, shared_output: &RefCell<String>, replace_marker: &str) {
        loop {
            let mut output = self.source.output().await;

            if output.is_empty() {
                output = "n/a".to_string();
            }

            *shared_output.borrow_mut() = self.format.replace(replace_marker, &output);

            tokio::time::sleep(Duration::from_secs(self.sec)).await;
        }
    }
}
