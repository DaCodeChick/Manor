use bitflags::bitflags;

bitflags! {
    /// Flags for the connection state.
    pub struct ConnectionFlags: u8 {
        const AWAIT_DISCONNECT = 1;
    }
}

/// Represents a connection with its state and statistics.
#[derive(Debug)]
pub struct Connection {
    flags: ConnectionFlags,
    data_waiting: u32,
    bytes_sent: u32,
}

impl Connection {
    /// Returns the amount of data waiting to be sent.
    pub const fn check_data_waiting(&self) -> u32 {
        self.data_waiting
    }

    /// Flags the connection as awaiting a disconnect.
    pub fn flag_disconnect(&mut self) {
        self.flags.insert(ConnectionFlags::AWAIT_DISCONNECT);
    }
}
