use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::error::{Error, Result};

pub async fn read_line(from: &str) -> Result<String> {
    let file = File::open(from).await.map_err(|e| Error::io(from, e))?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .await
        .map_err(|e| Error::io(from, format!("read line: {}", e)))?;

    Ok(buf.trim().to_string())
}

pub async fn read_lines(from: &str, num_lines: usize) -> Result<String> {
    let file = File::open(from).await.map_err(|e| Error::io(from, e))?;
    let mut reader = BufReader::new(file);

    // Pre-allocate buffer: estimate ~64 bytes per line for typical proc files
    let estimated_capacity = num_lines.saturating_mul(64);
    let mut buf = String::with_capacity(estimated_capacity);

    for _ in 0..num_lines {
        let bytes_read = reader
            .read_line(&mut buf)
            .await
            .map_err(|e| Error::io(from, format!("read file: {}", e)))?;

        if bytes_read == 0 {
            break;
        }
    }

    Ok(buf)
}

pub fn rounded_percent(numerator: u64, denominator: u64) -> Result<u64> {
    if denominator == 0 {
        return Err(Error::calculation(
            "percent calculation with zero denominator",
        ));
    }

    let per_mille = (numerator as u128 * 1000 / denominator as u128) as u64;
    let percent = per_mille.saturating_add(5) / 10;
    let capped = percent.min(100);

    Ok(capped)
}

#[cfg(test)]
mod tests {
    use super::rounded_percent;

    #[test]
    fn rounds_to_nearest_percent() {
        assert_eq!(rounded_percent(425, 1000).unwrap(), 43);
        assert_eq!(rounded_percent(424, 1000).unwrap(), 42);
    }

    #[test]
    fn caps_at_hundred() {
        assert_eq!(rounded_percent(150, 100).unwrap(), 100);
    }

    #[test]
    fn zero_numerator_is_zero() {
        assert_eq!(rounded_percent(0, 100).unwrap(), 0);
    }

    #[test]
    fn zero_denominator_errors() {
        assert!(rounded_percent(1, 0).is_err());
    }
}
