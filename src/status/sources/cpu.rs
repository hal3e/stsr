use crate::status::{
    Error, Result,
    utils::{read_line, rounded_percent},
};

const PROC_STAT_PATH: &str = "/proc/stat";

#[derive(Default, Debug)]
pub struct Cpu {
    previous: Option<CpuStat>,
}

impl Cpu {
    pub async fn cpu_percent(&mut self) -> Result<String> {
        let line = read_line(PROC_STAT_PATH).await?;
        let cpu_stat = line.parse::<CpuStat>()?;

        let output = match self.previous {
            None => {
                // First read: no previous data to compare against
                self.previous = Some(cpu_stat);
                return Ok("0".to_string());
            }
            Some(ref prev) => {
                let diff_sum_all = cpu_stat.sum_all().saturating_sub(prev.sum_all());
                let diff_sum = cpu_stat.sum().saturating_sub(prev.sum());

                if diff_sum_all == 0 {
                    Err(Error::calculation(format!(
                        "invalid stat delta: total={diff_sum_all}, active={diff_sum}"
                    )))
                } else {
                    rounded_percent(diff_sum, diff_sum_all).map(|num| num.to_string())
                }
            }
        };

        self.previous = Some(cpu_stat);

        output
    }
}

#[derive(Default, Debug)]
struct CpuStat {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
}

impl CpuStat {
    const fn sum_all(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq
    }

    const fn sum(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq
    }
}

impl std::str::FromStr for CpuStat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.trim_start_matches("cpu").split_whitespace();

        let mut next_value = |name: &str| -> std::result::Result<u64, Error> {
            let value = parts
                .next()
                .ok_or_else(|| Error::parse(PROC_STAT_PATH, format!("missing `{name}` field")))?;

            value.parse::<u64>().map_err(|err| {
                Error::parse(PROC_STAT_PATH, format!("invalid `{name}` value: {err}"))
            })
        };

        Ok(Self {
            user: next_value("user")?,
            nice: next_value("nice")?,
            system: next_value("system")?,
            idle: next_value("idle")?,
            iowait: next_value("iowait")?,
            irq: next_value("irq")?,
            softirq: next_value("softirq")?,
        })
    }
}
