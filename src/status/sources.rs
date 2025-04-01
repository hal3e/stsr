use chrono::Utc;
use chrono_tz::Europe::Vienna;

use super::utils::read_line;

mod command;
mod cpu;
mod ram;

pub enum Source {
    Command { cmd: String, args: Vec<String> },
    Cpu(cpu::Cpu),
    Battery { name: String },
    Ram,
    DateTime { format: String },
}

impl Source {
    pub fn cpu() -> Self {
        Self::Cpu(cpu::Cpu::default())
    }
}

impl Source {
    pub async fn output(&mut self) -> String {
        match self {
            Source::Command { cmd, args } => command::run(cmd, args).await,
            Source::Cpu(cpu) => cpu.cpu_percent().await,
            Source::Battery { name } => {
                read_line(&format!("/sys/class/power_supply/{name}/capacity")).await
            }
            Source::Ram => ram::ram_percent().await,
            Source::DateTime { format } => {
                Utc::now().with_timezone(&Vienna).format(format).to_string()
            }
        }
    }
}
