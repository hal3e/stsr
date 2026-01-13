use crate::{
    config::COMMAND_TIMEOUT,
    status::{Error, Result},
};

pub async fn run(cmd: &str, args: &[&str]) -> Result<String> {
    let mut command = tokio::process::Command::new(cmd);
    command.kill_on_drop(true).args(args);

    let full_command = if args.is_empty() {
        cmd.to_string()
    } else {
        format!("{} {}", cmd, args.join(" "))
    };

    match tokio::time::timeout(COMMAND_TIMEOUT, command.output()).await {
        Ok(Ok(res)) => {
            if !res.status.success() {
                let stderr = String::from_utf8_lossy(&res.stderr);

                return Err(Error::CommandFailed {
                    command: full_command,
                    status: res.status.to_string(),
                    stderr: stderr.trim().to_string(),
                });
            }

            match String::from_utf8(res.stdout) {
                Ok(stdout) => Ok(stdout.trim().to_string()),
                Err(_) => Err(Error::Utf8Decode {
                    context: format!("stdout from command '{}'", full_command),
                }),
            }
        }
        Ok(Err(err)) => Err(Error::CommandFailed {
            command: full_command,
            status: "spawn failed".to_string(),
            stderr: err.to_string(),
        }),
        Err(_) => Err(Error::CommandTimeout {
            command: full_command,
            timeout_secs: COMMAND_TIMEOUT.as_secs(),
        }),
    }
}
