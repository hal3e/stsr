use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn read_line(from: &str) -> String {
    let Ok(file) = File::open(from).await else {
        eprintln!("could not open file `{from}`");

        return "err".to_string();
    };

    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    if let Err(err) = reader.read_line(&mut buf).await {
        eprintln!("error reading line from `{from}`: {err}");

        return "err".to_string();
    }

    buf.trim().to_string()
}

pub async fn read_n_lines(from: &str, num_lines: usize) -> String {
    let Ok(file) = File::open(from).await else {
        eprintln!("could not open file `{from}`");

        return "err".to_string();
    };

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
                eprintln!("error reading file `{from}`: {err}");

                return "err".to_string();
            }
        }
    }

    buf
}

pub fn rounded_percent(numerator: u64, denominator: u64) -> Option<String> {
    if denominator == 0 {
        return None;
    }

    let per_mille = numerator.saturating_mul(1000) / denominator;
    let percent = per_mille.saturating_add(5) / 10;
    let capped = percent.min(100);

    Some(capped.to_string())
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
    fn zero_denominator() {
        assert!(rounded_percent(1, 0).is_none());
    }
}
