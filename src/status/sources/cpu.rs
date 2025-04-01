use crate::status::utils::read_line;

#[derive(Default)]
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
                eprintln!("error parsing cpu stat from `{}`: {}", path, err);

                return "err".to_string();
            }
        };

        let diff_sum_all = self.previous.sum_all() - cpu_stat.sum_all();
        let diff_sum = self.previous.sum() - cpu_stat.sum();

        self.previous = cpu_stat;

        format!("{:.0}", (100.0 * diff_sum / diff_sum_all).round())
    }
}

#[derive(Default)]
struct CpuStat {
    user: u32,
    nice: u32,
    system: u32,
    idle: u32,
    iowait: u32,
    irq: u32,
    softirq: u32,
}

impl CpuStat {
    fn sum_all(&self) -> f64 {
        f64::from(self.user)
            + f64::from(self.nice)
            + f64::from(self.system)
            + f64::from(self.idle)
            + f64::from(self.iowait)
            + f64::from(self.irq)
            + f64::from(self.softirq)
    }

    fn sum(&self) -> f64 {
        f64::from(self.user)
            + f64::from(self.nice)
            + f64::from(self.system)
            + f64::from(self.irq)
            + f64::from(self.softirq)
    }
}

impl std::str::FromStr for CpuStat {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measures: Vec<&str> = s
            .trim_start_matches("cpu")
            .trim_start()
            .split(' ')
            .collect();

        Ok(CpuStat {
            user: measures[0].parse()?,
            nice: measures[1].parse()?,
            system: measures[2].parse()?,
            idle: measures[3].parse()?,
            iowait: measures[4].parse()?,
            irq: measures[5].parse()?,
            softirq: measures[6].parse()?,
        })
    }
}
