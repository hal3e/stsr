use x11rb::{
    connection::Connection,
    protocol::xproto::{AtomEnum, ConnectionExt, PropMode},
    rust_connection::RustConnection,
};

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct X11rb {
    connection: RustConnection,
    root_window: u32,
    name_atom: u32,
    consecutive_failures: u32,
    max_failures_before_reconnect: u32,
}

impl X11rb {
    pub fn new(max_failures_before_reconnect: u32) -> Result<Self> {
        Self::connect(max_failures_before_reconnect)
    }

    fn connect(max_failures_before_reconnect: u32) -> Result<Self> {
        let (connection, screen_num) =
            x11rb::connect(None).map_err(|err| Error::x11(format!("connect: {}", err)))?;
        let screen = &connection.setup().roots[screen_num];
        let root_window = screen.root;

        let name_atom = connection
            .intern_atom(false, b"WM_NAME")
            .map_err(|err| Error::x11(format!("intern atom: {}", err)))?
            .reply()
            .map_err(|err| Error::x11(format!("intern atom reply: {}", err)))?
            .atom;

        Ok(Self {
            connection,
            root_window,
            name_atom,
            consecutive_failures: 0,
            max_failures_before_reconnect,
        })
    }

    fn reconnect(&mut self) -> Result<()> {
        let max_failures = self.max_failures_before_reconnect;
        *self = Self::connect(max_failures)?;
        Ok(())
    }

    pub fn set_root_win_name(&mut self, name: &str) -> Result<()> {
        match self.try_set_root_win_name(name) {
            Ok(()) => {
                self.consecutive_failures = 0;
                Ok(())
            }
            Err(err) => {
                self.consecutive_failures += 1;
                eprintln!("error writing root window name: {err}");

                if self.consecutive_failures >= self.max_failures_before_reconnect {
                    eprintln!(
                        "X11 write failed {} times consecutively, attempting reconnect...",
                        self.consecutive_failures
                    );

                    match self.reconnect() {
                        Ok(()) => {
                            eprintln!("X11 reconnection successful");
                            // Try writing again after successful reconnect
                            self.try_set_root_win_name(name)
                        }
                        Err(reconnect_err) => {
                            eprintln!("X11 reconnection failed: {reconnect_err}");
                            Err(reconnect_err)
                        }
                    }
                } else {
                    Err(err)
                }
            }
        }
    }

    fn try_set_root_win_name(&self, name: &str) -> Result<()> {
        self.connection
            .change_property(
                PropMode::REPLACE,
                self.root_window,
                self.name_atom,
                AtomEnum::STRING,
                8,
                name.len()
                    .try_into()
                    .map_err(|err| Error::x11(format!("name length conversion: {}", err)))?,
                name.as_bytes(),
            )
            .map_err(|err| Error::x11(format!("change property: {}", err)))?;
        self.connection
            .flush()
            .map_err(|err| Error::x11(format!("flush: {}", err)))?;

        Ok(())
    }
}
