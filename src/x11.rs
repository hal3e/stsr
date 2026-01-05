use x11rb::{
    connection::Connection,
    protocol::xproto::{AtomEnum, ConnectionExt, PropMode},
    rust_connection::RustConnection,
};

use crate::status::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct X11rb {
    connection: RustConnection,
    root_window: u32,
    name_atom: u32,
}

impl X11rb {
    pub fn new() -> Result<Self> {
        let (connection, screen_num) = x11rb::connect(None)
            .map_err(|err| Error::x11(format!("connect: {}", err)))?;
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
        })
    }

    pub fn set_root_win_name(&self, name: &str) -> Result<()> {
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
