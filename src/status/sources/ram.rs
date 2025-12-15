use crate::status::utils::{read_n_lines, rounded_percent};

pub async fn ram_percent() -> String {
    let path = "/proc/meminfo";
    let line = read_n_lines(path, 5).await;
    let ram_stat = match line.parse::<RamStat>() {
        Ok(ram_stat) => ram_stat,
        Err(err) => {
            eprintln!("error parsing ram stat from `{path}`: {err}");

            return "err".to_string();
        }
    };

    let available = ram_stat.available;

    let used = ram_stat.total.saturating_sub(available);
    match rounded_percent(used, ram_stat.total) {
        Some(percent) => percent,
        None => {
            eprintln!("invalid total memory size read from `{path}`");
            "err".to_string()
        }
    }
}

#[derive(Default)]
struct RamStat {
    total: u64,
    available: u64,
}

impl std::str::FromStr for RamStat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ram_stat = Self::default();

        for line in s.lines() {
            if let Some((key, value_part)) = line.split_once(':') {
                let key = key.trim();
                let value_str = value_part.split_whitespace().next().unwrap_or("");
                let value = value_str
                    .parse::<u64>()
                    .map_err(|err| format!("invalid value for `{key}`: {err}"))?;

                match key {
                    "MemTotal" => ram_stat.total = value,
                    "MemAvailable" => ram_stat.available = value,
                    _ => {}
                }
            }
        }

        if ram_stat.total == 0 {
            return Err("missing `MemTotal` in /proc/meminfo".to_string());
        }
        if ram_stat.available == 0 {
            return Err("missing `MemAvailable` in /proc/meminfo".to_string());
        }

        Ok(ram_stat)
    }
}
