use crate::status::utils::read_line;

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

        let diff_sum_all = self.previous.sum_all() - cpu_stat.sum_all();
        let diff_sum = self.previous.sum() - cpu_stat.sum();

        self.previous = cpu_stat;

        format!("{:.0}", (100.0 * diff_sum / diff_sum_all).round())
    }
}

#[derive(Default, Debug)]
struct CpuStat {
    user: f64,
    nice: f64,
    system: f64,
    idle: f64,
    iowait: f64,
    irq: f64,
    softirq: f64,
}

impl CpuStat {
    fn sum_all(&self) -> f64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq
    }

    fn sum(&self) -> f64 {
        self.user + self.nice + self.system + self.irq + self.softirq
    }
}

impl std::str::FromStr for CpuStat {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measures: Vec<&str> = s
            .trim_start_matches("cpu")
            .trim_start()
            .split(' ')
            .collect();

        Ok(Self {
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
