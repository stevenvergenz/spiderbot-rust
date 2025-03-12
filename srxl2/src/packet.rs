use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    TryFromBytes,
    IntoBytes,
};

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

pub mod handshake;
pub mod bind;
pub mod param;
pub mod rssi;
pub mod vtx;
pub mod telemetry;