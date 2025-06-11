use std::error::Error;

use x11rb::{connection::Connection, protocol::xproto::*, rust_connection::RustConnection};

pub struct X11rb {
    connection: RustConnection,
    root_window: u32,
    name_atom: u32,
}

impl X11rb {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (connection, screen_num) = x11rb::connect(None)?;
        let screen = &connection.setup().roots[screen_num];
        let root_window = screen.root;

        // Intern the _NET_WM_NAME atom
        let name_atom = connection.intern_atom(false, b"WM_NAME")?.reply()?.atom;

        Ok(X11rb {
            connection,
            root_window,
            name_atom,
        })
    }

    pub fn set_root_win_name(&self, name: &str) -> Result<(), Box<dyn Error>> {
        self.connection.change_property(
            PropMode::REPLACE,
            self.root_window,
            self.name_atom,
            AtomEnum::STRING,
            8,
            name.len() as u32,
            name.as_bytes(),
        )?;
        self.connection.flush()?;

        Ok(())
    }
}
