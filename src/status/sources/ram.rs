use crate::status::utils::read_n_lines;

pub async fn ram_percent() -> String {
    let path = "/proc/meminfo";
    let line = read_n_lines(path, 5).await;
    let ram_stat = match line.parse::<RamStat>() {
        Ok(ram_stat) => ram_stat,
        Err(err) => {
            eprintln!("error parsing ram stat from `{}`: {}", path, err);

            return "err".to_string();
        }
    };

    format!(
        "{:.0}",
        (100.0
            * f64::from((ram_stat.total - ram_stat.free) - (ram_stat.buffers + ram_stat.cached))
            / f64::from(ram_stat.total))
        .round()
    )
}

#[derive(Default)]
struct RamStat {
    total: u32,
    free: u32,
    buffers: u32,
    cached: u32,
}

impl std::str::FromStr for RamStat {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ram_stat = RamStat::default();

        for line in s.lines() {
            if let Some((key, value_part)) = line.split_once(':') {
                let key = key.trim();
                let value_str = value_part.split_whitespace().next().unwrap_or("");
                let value: u32 = value_str.parse()?;

                match key {
                    "MemTotal" => {
                        ram_stat.total = value;
                    }
                    "MemFree" => {
                        ram_stat.free = value;
                    }
                    "Buffers" => {
                        ram_stat.buffers = value;
                    }
                    "Cached" => {
                        ram_stat.cached = value;
                    }
                    _ => {}
                }
            }
        }

        Ok(ram_stat)
    }
}
