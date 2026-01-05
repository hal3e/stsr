use crate::status::{
    Error, Result,
    utils::{read_lines, rounded_percent},
};

const PROC_MEMINFO_PATH: &str = "/proc/meminfo";

pub async fn ram_percent() -> Result<String> {
    let lines = read_lines(PROC_MEMINFO_PATH, 5).await?;
    let ram_stat = lines.parse::<RamStat>()?;

    let available = ram_stat.available;
    let used = ram_stat.total.saturating_sub(available);

    rounded_percent(used, ram_stat.total).map(|num| num.to_string())
}

#[derive(Default)]
struct RamStat {
    total: u64,
    available: u64,
}

impl std::str::FromStr for RamStat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ram_stat = Self::default();

        for line in s.lines() {
            if let Some((key, value_part)) = line.split_once(':') {
                let key = key.trim();
                let value_str = value_part.split_whitespace().next().unwrap_or("");
                let value = value_str.parse::<u64>().map_err(|err| {
                    Error::parse(
                        PROC_MEMINFO_PATH,
                        format!("invalid value for `{key}`: {err}"),
                    )
                })?;

                match key {
                    "MemTotal" => ram_stat.total = value,
                    "MemAvailable" => ram_stat.available = value,
                    _ => {}
                }
            }
        }

        if ram_stat.total == 0 {
            return Err(Error::parse(PROC_MEMINFO_PATH, "missing `MemTotal`"));
        }

        Ok(ram_stat)
    }
}
