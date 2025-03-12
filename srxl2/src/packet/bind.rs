use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    TryFromBytes,
    IntoBytes,
};
use crate::{
    packet::Header,
    id::DeviceId,
};

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Request {
    Enter = 0xEB,
    Status = 0xB5,
    BoundData = 0xDB,
    SetBind = 0x5B,
}

/// Bit masks for Options byte
pub mod opt {
    pub const NONE: u8            = 0x00;
    /// Set if this device should be enabled as the current telemetry device to tx over RF
    pub const TELEM_TX_ENABLE: u8 = 0x01;
    /// Set if this device should reply to a bind request with a Discover packet over RF
    pub const BIND_TX_ENABLE: u8  = 0x02;
    /// Set if this device should request US transmit power levels instead of EU
    pub const US_POWER: u8        = 0x04;
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum BindStatus {
    NotBound            = 0x00,
    // Air types
    Dsm2_1024_22ms      = 0x01,
    Dsm2_1024Mc24       = 0x02,
    Dsm2_2048_11ms      = 0x12,
    Dsmx22Ms            = 0xA2,
    Dsmx11Ms            = 0xB2,
    // Surface types (corresponding Air type bitwise OR'd with 0x40)
    SurfaceDsm1         = 0x40,
    SurfaceDsm2_16p6ms  = 0x63,
    Dsmr11ms22ms        = 0xE2,
    Dsmr5p5ms           = 0xE4,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct BindData {
    pub bind_type: u8,
    pub options: u8,
    pub guid: u64,
    pub uid: u32,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct BindPacket {
    pub hdr: Header,
    pub request: Request,
    pub device_id: DeviceId,
    pub data: BindData,
    pub crc: u16,
}
