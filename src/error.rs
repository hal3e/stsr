use std::{error, fmt};

#[derive(Clone)]
pub enum Error {
    Io {
        path: String,
        message: String,
    },
    Parse {
        context: String,
        message: String,
    },
    CommandFailed {
        command: String,
        status: String,
        stderr: String,
    },
    CommandTimeout {
        command: String,
        timeout_secs: u64,
    },
    Utf8Decode {
        context: String,
    },
    Calculation {
        message: String,
    },
    Config {
        message: String,
    },
    X11 {
        message: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create an I/O error with file path context
    pub fn io(path: impl Into<String>, message: impl fmt::Display) -> Self {
        Error::Io {
            path: path.into(),
            message: message.to_string(),
        }
    }

    /// Create a parse error with context
    pub fn parse(context: impl Into<String>, message: impl Into<String>) -> Self {
        Error::Parse {
            context: context.into(),
            message: message.into(),
        }
    }

    /// Create a calculation error
    pub fn calculation(message: impl Into<String>) -> Self {
        Error::Calculation {
            message: message.into(),
        }
    }

    /// Create an X11 error
    pub fn x11(message: impl fmt::Display) -> Self {
        Error::X11 {
            message: message.to_string(),
        }
    }

    /// Create a config error
    pub fn config(message: impl Into<String>) -> Self {
        Error::Config {
            message: message.into(),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, message } => write!(f, "i/o; '{}': {}", path, message),
            Error::Parse { context, message } => {
                write!(f, "parse: {}: {}", context, message)
            }
            Error::CommandFailed {
                command,
                status,
                stderr,
            } => {
                write!(f, "`{}` failed with status {}: {}", command, status, stderr)
            }
            Error::CommandTimeout {
                command,
                timeout_secs,
            } => {
                write!(f, "`{}` timed out after {}s", command, timeout_secs)
            }
            Error::Utf8Decode { context } => write!(f, "utf-8 decode: {}", context),
            Error::Calculation { message } => write!(f, "calculation: {}", message),
            Error::Config { message } => write!(f, "config: {}", message),
            Error::X11 { message } => write!(f, "X11: {}", message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl error::Error for Error {}
