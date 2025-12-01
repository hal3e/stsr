pub async fn run(cmd: &str, args: &[&str]) -> String {
    let mut command = tokio::process::Command::new(cmd);
    command.args(args);

    match command.output().await {
        Ok(res) => String::from_utf8(res.stdout)
            .unwrap_or_else(|_| "err".to_string())
            .trim()
            .to_string(),
        Err(err) => {
            eprintln!("error running command `{cmd}`: {err}");

            "err".to_string()
        }
    }
}
