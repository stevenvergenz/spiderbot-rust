use zerocopy::{
    KnownLayout,
    Immutable,
    TryFromBytes,
    IntoBytes,
};
use crate::{
    packet::Header,
    device::DeviceId,
};

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Request {
    Query = 0x50,
    Write = 0x57,
}

/// Parameter Config
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct ParamPayload {
    pub request: Request,
    pub dest_dev_id: DeviceId,
    pub param_id: u32,
    pub param_val: u32,
    pub crc: u16,
}
