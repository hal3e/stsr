use crate::status::utils::{read_line, rounded_percent};

#[derive(Default, Debug)]
pub struct Cpu {
    previous: CpuStat,
}

impl Cpu {
    pub async fn cpu_percent(&mut self) -> String {
        let path = "/proc/stat";
        let line = read_line(path).await;

        let cpu_stat = match line.parse::<CpuStat>() {
            Ok(cpu_stat) => cpu_stat,
            Err(err) => {
                eprintln!("error parsing cpu stat from `{path}`: {err}");

                return "err".to_string();
            }
        };

        let diff_sum_all = cpu_stat.sum_all().saturating_sub(self.previous.sum_all());
        let diff_sum = cpu_stat.sum().saturating_sub(self.previous.sum());

        let output = if diff_sum_all == 0 {
            eprintln!("invalid cpu stat delta: total={diff_sum_all}, active={diff_sum}");
            "err".to_string()
        } else {
            match rounded_percent(diff_sum, diff_sum_all) {
                Some(percent) => percent,
                None => {
                    eprintln!("invalid cpu stat percentage calculation");
                    "err".to_string()
                }
            }
        };

        self.previous = cpu_stat;

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
    fn sum_all(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq
    }

    fn sum(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq
    }
}

impl std::str::FromStr for CpuStat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim_start_matches("cpu").split_whitespace();

        let mut next_value = |name: &str| -> Result<u64, Self::Err> {
            let value = parts
                .next()
                .ok_or_else(|| format!("missing `{name}` field in /proc/stat"))?;

            value
                .parse::<u64>()
                .map_err(|err| format!("invalid `{name}` value in /proc/stat: {err}"))
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
