#[derive(Debug)]
pub enum PacketCastError {
    HeaderMismatch,
    Cast,
}

impl core::fmt::Display for PacketCastError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

impl core::error::Error for PacketCastError { }
