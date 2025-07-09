use embedded_io::{
    ErrorKind,
    Error,
};
use ufmt::uDisplay;

#[derive(Debug)]
pub struct SerialError {
    kind: ErrorKind,
}

impl SerialError {
    pub fn new(kind: ErrorKind) -> Self {
        SerialError { kind }
    }
}

impl Error for SerialError {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl uDisplay for SerialError {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error> where W: ufmt::uWrite + ?Sized {
        match self.kind {
            ErrorKind::InvalidData => ufmt::uwrite!(fmt, "Invalid data"),
            ErrorKind::BrokenPipe => ufmt::uwrite!(fmt, "Broken pipe"),
            ErrorKind::TimedOut => ufmt::uwrite!(fmt, "Timed out"),
            ErrorKind::NotConnected => ufmt::uwrite!(fmt, "Not connected"),
            _ => ufmt::uwrite!(fmt, "Unknown error"),
        }
    }
}