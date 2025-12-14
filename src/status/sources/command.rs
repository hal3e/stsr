pub async fn run(cmd: &str, args: &[&str]) -> String {
    let mut command = tokio::process::Command::new(cmd);
    command.args(args);

    match command.output().await {
        Ok(res) => {
            if !res.status.success() {
                let stderr = String::from_utf8_lossy(&res.stderr);
                eprintln!(
                    "command `{cmd}` failed with status {}: {}",
                    res.status,
                    stderr.trim()
                );
                return "err".to_string();
            }

            match String::from_utf8(res.stdout) {
                Ok(stdout) => stdout.trim().to_string(),
                Err(err) => {
                    eprintln!("error decoding stdout of `{cmd}`: {err}");
                    "err".to_string()
                }
            }
        }
        Err(err) => {
            eprintln!("error running command `{cmd}`: {err}");
            "err".to_string()
        }
    }
}
