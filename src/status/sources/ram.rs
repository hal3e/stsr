use crate::status::{
    Error, Result,
    utils::{read_lines, rounded_percent},
};

const PROC_MEMINFO_PATH: &str = "/proc/meminfo";
const MEMINFO_NUM_LINES: usize = 5;

pub async fn ram_percent() -> Result<String> {
    let lines = read_lines(PROC_MEMINFO_PATH, MEMINFO_NUM_LINES).await?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_meminfo() {
        let input = "MemTotal:       16000000 kB\n\
                     MemFree:         8000000 kB\n\
                     MemAvailable:   10000000 kB\n";
        let stat = input.parse::<RamStat>().unwrap();
        assert_eq!(stat.total, 16000000);
        assert_eq!(stat.available, 10000000);
    }

    #[test]
    fn parses_meminfo_with_extra_fields() {
        let input = "MemTotal:       16000000 kB\n\
                     Buffers:          500000 kB\n\
                     Cached:          2000000 kB\n\
                     MemAvailable:   10000000 kB\n\
                     SwapTotal:       8000000 kB\n";
        let stat = input.parse::<RamStat>().unwrap();
        assert_eq!(stat.total, 16000000);
        assert_eq!(stat.available, 10000000);
    }

    #[test]
    fn errors_on_missing_memtotal() {
        let input = "MemFree:         8000000 kB\n\
                     MemAvailable:   10000000 kB\n";
        let result = input.parse::<RamStat>();
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_zero_memtotal() {
        let input = "MemTotal:              0 kB\n\
                     MemAvailable:   10000000 kB\n";
        let result = input.parse::<RamStat>();
        assert!(result.is_err());
    }

    #[test]
    fn accepts_zero_memavailable() {
        let input = "MemTotal:       16000000 kB\n\
                     MemAvailable:          0 kB\n";
        let stat = input.parse::<RamStat>().unwrap();
        assert_eq!(stat.total, 16000000);
        assert_eq!(stat.available, 0);
    }

    #[test]
    fn accepts_missing_memavailable() {
        let input = "MemTotal:       16000000 kB\n\
                     MemFree:         8000000 kB\n";
        let stat = input.parse::<RamStat>().unwrap();
        assert_eq!(stat.total, 16000000);
        assert_eq!(stat.available, 0);
    }

    #[test]
    fn errors_on_invalid_values() {
        let input = "MemTotal:       abc kB\n\
                     MemAvailable:   10000000 kB\n";
        let result = input.parse::<RamStat>();
        assert!(result.is_err());
    }

    #[test]
    fn handles_values_without_units() {
        let input = "MemTotal:       16000000\n\
                     MemAvailable:   10000000\n";
        let stat = input.parse::<RamStat>().unwrap();
        assert_eq!(stat.total, 16000000);
        assert_eq!(stat.available, 10000000);
    }
}
