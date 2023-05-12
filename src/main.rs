use std::cell::RefCell;

use futures::future::join_all;
use tokio::{process::Command, sync::Mutex, time::Duration};

struct CommandConfig {
    cmd: String,
    args: Vec<String>,
    default: String,
    sec: u64,
}

fn get_config() -> Vec<CommandConfig> {
    vec![CommandConfig {
        cmd: "date".into(),
        args: vec!["+%T".into()],
        default: "N/A".to_string(),
        sec: 1,
    }]
}

struct Runner {
    cmd: Mutex<Command>,
    output: RefCell<String>,
    sec: u64,
}

impl Runner {
    async fn get_cmd_output(&self) -> String {
        let result = self.cmd.lock().await.output().await.unwrap();
        String::from_utf8(result.stdout).unwrap().trim().to_string()
    }

    fn update_output(&self, output: String) {
        *self.output.borrow_mut() = output;
    }

    async fn run(&self) {
        loop {
            let output = self.get_cmd_output().await;
            self.update_output(output);

            tokio::time::sleep(Duration::from_secs(self.sec)).await;
        }
    }
}

async fn write_output(runners: &[Runner]) {
    loop {
        for r in runners {
            print!("{}  ", r.output.borrow())
        }
        println!();

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let runners: Vec<Runner> = get_config()
        .into_iter()
        .map(
            |CommandConfig {
                 cmd,
                 args,
                 default,
                 sec,
             }| {
                let mut cmd = Command::new(cmd);
                cmd.args(args);

                Runner {
                    cmd: Mutex::new(cmd),
                    output: RefCell::new(default),
                    sec,
                }
            },
        )
        .collect();

    let futures = join_all(runners.iter().map(|r| r.run()));

    tokio::join!(futures, write_output(&runners));
}
