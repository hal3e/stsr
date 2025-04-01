pub async fn run(cmd: &str, args: &[String]) -> String {
    let mut ccmd = tokio::process::Command::new(cmd);
    ccmd.args(args);

    match ccmd.output().await {
        Ok(res) => String::from_utf8(res.stdout)
            .unwrap_or("err".to_string())
            .trim()
            .to_string(),
        Err(err) => {
            eprintln!("error running command `{}`: {}", cmd, err);

            "err".to_string()
        }
    }
}
