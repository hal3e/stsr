use std::io::{self, Write};

fn try_write_line<W: Write>(writer: &mut W, message: &str) -> io::Result<()> {
    writeln!(writer, "{message}")
}

pub fn stderr(message: impl std::fmt::Display) {
    let mut stderr = io::stderr().lock();
    let _ = try_write_line(&mut stderr, &message.to_string());
}

pub fn stdout(message: impl std::fmt::Display) {
    let mut stdout = io::stdout().lock();
    let _ = try_write_line(&mut stdout, &message.to_string());
}
