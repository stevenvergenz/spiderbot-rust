use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    TryFromBytes,
    IntoBytes,
    CastError,
    TryCastError,
};

pub const SRXL_MAX_BUFFER_SIZE: usize = 80;

/// Spektrum SRXL header
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct Header {
    /// Always 0xKA6 for SRXL2
    pub srxl_id: u8,
    pub packet_type: u8,
    pub length: u8,
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum PacketType {
    /// Handshake packet
    Handshake = 0x21,
    /// Bind info packet
    BindInfo = 0x41,
    /// Parameter config packet
    ParamConfig = 0x50,
    /// Signal quality packet
    SignalQuality = 0x55,
    /// Telemetry data packet
    TelemetryData = 0x80,
    /// Control data packet
    ControlData = 0xCD,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct Packet {
    pub raw: [u8; SRXL_MAX_BUFFER_SIZE],
}

impl Packet {
    pub fn header(&self) -> Result<&Header, CastError<&[u8], Header>> {
        Header::ref_from_bytes(self.raw.as_slice())
    }
}

pub mod handshake;
pub mod bind;
pub mod param;
pub mod rssi;
pub mod vtx;
pub mod telemetry;
pub mod fwd_pgm;
pub mod channel;
pub mod control;