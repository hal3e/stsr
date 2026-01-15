use std::time::Duration;

use crate::status::{Error, Result};

/// Runs a command with a per-run timeout in seconds.
pub async fn run(cmd: &str, args: &[&str], timeout: u64) -> Result<String> {
    let mut cmd_tokio = tokio::process::Command::new(cmd);
    cmd_tokio.kill_on_drop(true).args(args);

    let command = if args.is_empty() {
        cmd.to_string()
    } else {
        format!("{} {}", cmd, args.join(" "))
    };

    match tokio::time::timeout(Duration::from_secs(timeout), cmd_tokio.output()).await {
        Ok(Ok(res)) => {
            if !res.status.success() {
                let stderr = String::from_utf8_lossy(&res.stderr);

                return Err(Error::CommandFailed {
                    command,
                    status: res.status.to_string(),
                    stderr: stderr.trim().to_string(),
                });
            }

            match String::from_utf8(res.stdout) {
                Ok(stdout) => Ok(stdout.trim().to_string()),
                Err(_) => Err(Error::Utf8Decode {
                    context: format!("stdout from command '{}'", command),
                }),
            }
        }
        Ok(Err(err)) => Err(Error::CommandFailed {
            command,
            status: "spawn failed".to_string(),
            stderr: err.to_string(),
        }),
        Err(_) => Err(Error::CommandTimeout { command, timeout }),
    }
}
