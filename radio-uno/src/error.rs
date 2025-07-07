use embedded_io::{
    ErrorKind,
    Error,
};

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