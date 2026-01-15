use chrono::Utc;
use chrono_tz::Tz;

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
        /// Timeout in seconds for the spawned process; on timeout returns `err`.
        timeout: u64,
    },
    Shell {
        script: &'static str,
        /// Timeout in seconds for the spawned process; on timeout returns `err`.
        timeout: u64,
    },
    Cpu(cpu::Cpu),
    Battery {
        name: &'static str,
    },
    Ram,
    DateTime {
        format: &'static str,
        timezone: Tz,
    },
}

impl Source {
    pub fn cpu() -> Self {
        Self::Cpu(cpu::Cpu::default())
    }

    pub fn label(&self) -> String {
        match self {
            Self::Command { .. } => "command".to_string(),
            Self::Shell { .. } => "shell".to_string(),
            Self::Cpu(_) => "cpu".to_string(),
            Self::Battery { name } => format!("battery `{name}`"),
            Self::Ram => "ram".to_string(),
            Self::DateTime { format, .. } => format!("datetime `{format}`"),
        }
    }
}

impl Source {
    pub async fn output(&mut self) -> Result<String> {
        match self {
            Self::Command { cmd, args, timeout } => command::run(cmd, args, *timeout).await,
            Self::Shell {
                script,
                timeout: timeout_secs,
            } => command::run("sh", &["-c", script], *timeout_secs).await,
            Self::Cpu(cpu) => cpu.cpu_percent().await,
            Self::Battery { name } => {
                read_line(&format!("/sys/class/power_supply/{name}/capacity")).await
            }
            Self::Ram => ram::ram_percent().await,
            Self::DateTime { format, timezone } => Ok(Utc::now()
                .with_timezone(timezone)
                .format(format)
                .to_string()),
        }
    }
}
