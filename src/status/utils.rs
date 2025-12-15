use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use super::{Error, Result};

pub async fn read_line(from: &str) -> Result<String> {
    let file = File::open(from)
        .await
        .map_err(|err| Error(format!("open `{from}`: {err}")))?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .await
        .map_err(|err| Error(format!("read line from `{from}`: {err}")))?;

    Ok(buf.trim().to_string())
}

pub async fn read_n_lines(from: &str, num_lines: usize) -> Result<String> {
    let file = File::open(from)
        .await
        .map_err(|err| Error(format!("open `{from}`: {err}")))?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    for _ in 0..num_lines {
        match reader.read_line(&mut buf).await {
            Ok(num_bytes_read) => {
                if num_bytes_read == 0 {
                    break;
                }
            }
            Err(err) => {
                return Err(Error(format!("read file `{from}`: {err}")));
            }
        }
    }

    Ok(buf)
}

pub fn rounded_percent(numerator: u64, denominator: u64) -> Result<String> {
    if denominator == 0 {
        return Err(Error(
            "percent calculation with zero denominator".to_string(),
        ));
    }

    let per_mille = numerator.saturating_mul(1000) / denominator;
    let percent = per_mille.saturating_add(5) / 10;
    let capped = percent.min(100);

    Ok(capped.to_string())
}

#[cfg(test)]
mod tests {
    use super::rounded_percent;

    #[test]
    fn rounds_to_nearest_percent() {
        assert_eq!(rounded_percent(425, 1000).unwrap(), "43");
        assert_eq!(rounded_percent(424, 1000).unwrap(), "42");
    }

    #[test]
    fn caps_at_hundred() {
        assert_eq!(rounded_percent(150, 100).unwrap(), "100");
    }

    #[test]
    fn zero_numerator_is_zero() {
        assert_eq!(rounded_percent(0, 100).unwrap(), "0");
    }

    #[test]
    fn zero_denominator_errors() {
        assert!(rounded_percent(1, 0).is_err());
    }
}
