use std::time::Duration;

use crate::status::{Error, Result};

/// Maximum time to wait for a command to complete before timing out
const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);

pub async fn run(cmd: &str, args: &[&str]) -> Result<String> {
    let mut command = tokio::process::Command::new(cmd);
    command.args(args);

    match tokio::time::timeout(COMMAND_TIMEOUT, command.output()).await {
        Ok(Ok(res)) => {
            if !res.status.success() {
                let stderr = String::from_utf8_lossy(&res.stderr);

                return Err(Error(format!(
                    "failed with status {}: {}",
                    res.status,
                    stderr.trim()
                )));
            }

            match String::from_utf8(res.stdout) {
                Ok(stdout) => Ok(stdout.trim().to_string()),
                Err(err) => Err(Error(format!("decode stdout: {err}"))),
            }
        }
        Ok(Err(err)) => Err(Error(format!("run: {err}"))),
        Err(_) => Err(Error(format!(
            "command timed out after {:?}",
            COMMAND_TIMEOUT
        ))),
    }
}
