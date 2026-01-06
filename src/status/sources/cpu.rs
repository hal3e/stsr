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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_cpu_stat() {
        let stat = "cpu  1000 200 300 5000 100 50 25"
            .parse::<CpuStat>()
            .unwrap();
        assert_eq!(stat.user, 1000);
        assert_eq!(stat.nice, 200);
        assert_eq!(stat.system, 300);
        assert_eq!(stat.idle, 5000);
        assert_eq!(stat.iowait, 100);
        assert_eq!(stat.irq, 50);
        assert_eq!(stat.softirq, 25);
    }

    #[test]
    fn parses_cpu_stat_with_prefix() {
        let stat = "cpu  1000 200 300 5000 100 50 25"
            .parse::<CpuStat>()
            .unwrap();
        assert_eq!(stat.user, 1000);
    }

    #[test]
    fn errors_on_missing_fields() {
        let result = "cpu  1000 200".parse::<CpuStat>();
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_invalid_values() {
        let result = "cpu  abc 200 300 5000 100 50 25".parse::<CpuStat>();
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_negative_values() {
        let result = "cpu  -100 200 300 5000 100 50 25".parse::<CpuStat>();
        assert!(result.is_err());
    }

    #[test]
    fn sum_all_includes_all_fields() {
        let stat = CpuStat {
            user: 100,
            nice: 10,
            system: 50,
            idle: 1000,
            iowait: 20,
            irq: 5,
            softirq: 3,
        };
        assert_eq!(stat.sum_all(), 1188);
    }

    #[test]
    fn sum_excludes_idle_and_iowait() {
        let stat = CpuStat {
            user: 100,
            nice: 10,
            system: 50,
            idle: 1000,
            iowait: 20,
            irq: 5,
            softirq: 3,
        };
        // sum = user + nice + system + irq + softirq
        assert_eq!(stat.sum(), 168);
    }

    #[test]
    fn sum_with_zero_values() {
        let stat = CpuStat::default();
        assert_eq!(stat.sum_all(), 0);
        assert_eq!(stat.sum(), 0);
    }
}
