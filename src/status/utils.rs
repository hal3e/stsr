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
