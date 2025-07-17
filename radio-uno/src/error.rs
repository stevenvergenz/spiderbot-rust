use embedded_io::{
    ErrorKind,
    Error,
};
use ufmt::uDisplay;

#[derive(Debug)]
pub enum SerialError {
    BufferFull,
    NotImplemented,
}

impl Error for SerialError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::BufferFull => ErrorKind::OutOfMemory,
            Self::NotImplemented => ErrorKind::Unsupported
        }
    }
}

impl uDisplay for SerialError {
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error> where W: ufmt::uWrite + ?Sized {
        match self {
            SerialError::BufferFull => ufmt::uwrite!(fmt, "Buffer full"),
            SerialError::NotImplemented => ufmt::uwrite!(fmt, "Not implemented"),
        }
    }
}