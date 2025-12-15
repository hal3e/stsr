use crate::status::{Error, Result};

pub async fn run(cmd: &str, args: &[&str]) -> Result<String> {
    let mut command = tokio::process::Command::new(cmd);
    command.args(args);

    match command.output().await {
        Ok(res) => {
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
        Err(err) => Err(Error(format!("run: {err}"))),
    }
}
