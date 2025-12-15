use chrono::Utc;
use chrono_tz::Europe::Vienna;

use super::utils::read_line;
use crate::status::Result;

mod command;
mod cpu;
mod ram;

#[derive(Debug)]
pub enum Source {
    Command {
        cmd: &'static str,
        args: &'static [&'static str],
    },
    Cpu(cpu::Cpu),
    Battery {
        name: &'static str,
    },
    Ram,
    DateTime {
        format: &'static str,
    },
}

impl Source {
    pub fn cpu() -> Self {
        Self::Cpu(cpu::Cpu::default())
    }

    pub fn label(&self) -> String {
        match self {
            Self::Command { cmd, .. } => format!("command `{cmd}`"),
            Self::Cpu(_) => "cpu".to_string(),
            Self::Battery { name } => format!("battery `{name}`"),
            Self::Ram => "ram".to_string(),
            Self::DateTime { format } => format!("datetime `{format}`"),
        }
    }
}

impl Source {
    pub async fn output(&mut self) -> Result<String> {
        match self {
            Self::Command { cmd, args } => command::run(cmd, args).await,
            Self::Cpu(cpu) => cpu.cpu_percent().await,
            Self::Battery { name } => {
                read_line(&format!("/sys/class/power_supply/{name}/capacity")).await
            }
            Self::Ram => ram::ram_percent().await,
            Self::DateTime { format } => {
                Ok(Utc::now().with_timezone(&Vienna).format(format).to_string())
            }
        }
    }
}
