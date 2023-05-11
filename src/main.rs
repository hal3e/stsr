use futures::future::join_all;
use std::cell::RefCell;
use tokio::{process::Command, time::Duration};

struct CommandConfig {
    cmd: String,
    args: Vec<String>,
    sec: u64,
}

fn get_config() -> Vec<CommandConfig> {
    vec![CommandConfig {
        cmd: "date".into(),
        args: vec!["+%T".into()],
        sec: 1,
    }]
}

struct Runner {
    cmd: RefCell<Command>,
    cell: RefCell<String>,
    sec: u64,
}

impl Runner {
    fn new(cmd: Command, sec: u64) -> Self {
        Self {
            cmd: RefCell::new(cmd),
            cell: RefCell::new("".to_string()),
            sec,
        }
    }

    async fn run(&self) {
        loop {
            let result = self.cmd.borrow_mut().output().await.unwrap();
            *self.cell.borrow_mut() = String::from_utf8(result.stdout).unwrap().trim().to_string();

            tokio::time::sleep(Duration::from_secs(self.sec)).await;
        }
    }
}

async fn read_output(runners: &[Runner]) {
    loop {
        for r in runners {
            print!("{}  ", r.cell.borrow())
        }

        println!();
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let runners: Vec<Runner> = get_config()
        .into_iter()
        .map(|config| {
            let mut c = Command::new(config.cmd);
            c.args(config.args);
            Runner::new(c, config.sec)
        })
        .collect();

    let futures = join_all(runners.iter().map(|r| r.run()));

    tokio::join!(futures, read_output(&runners));
}
