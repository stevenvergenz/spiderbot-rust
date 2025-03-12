use zerocopy::{
    KnownLayout,
    Immutable,
    TryFromBytes,
    IntoBytes,
};
use super::Header;

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Request {
    Request = 0x52,
    Send = 0x53,
}

/// Signal Quality
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct RssiPacket {
    pub hdr: Header,
    pub request: Request,
    pub antenna_a: i8,
    pub antenna_b: i8,
    pub antenna_c: i8,
    pub antenna_d: i8,
    pub crc: u16,
}
