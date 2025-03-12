use zerocopy::{KnownLayout, Immutable, TryFromBytes, IntoBytes};
use super::Header;


/// Supported additional baud rates besides default 115200
/// NOTE: Treated as bitmask, ANDed with baud rates from slaves
pub mod baud {
    pub const BAUD_115200: u8 = 0;
    pub const BAUD_400000: u8 = 1;
}

/// Bit masks for Device Info byte sent via Handshake
pub mod dev_info {
    /// This is the base for non-RF devices
    pub const NO_RF: u8            = 0;
    /// This bit is set if the device is actively configured to transmit telemetry over RF
    pub const TELEM_TX_ENABLED: u8 = 1;
    /// This bit is set if the device can send full-range telemetry over RF
    pub const TELEM_FULL_RANGE: u8 = 2;
    /// This bit is set if the device supports Forward Programming via RF or SRXL
    pub const FWD_PROG_SUPPORT: u8 = 4;
}

/// Handshake
#[repr(C,packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct HandshakeData {
    pub src_dev_id: u8,
    pub dest_dev_id: u8,
    pub priority: u8,
    /// 0 = 115200, 1 = 400000 (See SRXL_BAUD_xxx definitions above)
    pub baud_supported: u8,
    /// See SRXL_DEVINFO_xxx definitions above for defined bits
    pub info: u8,
    /// Unique/random id to allow detection of two devices on bus with same deviceID
    pub uid: u32,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct HandshakePacket {
    pub hdr: Header,
    pub payload: HandshakeData,
    pub crc: u16,
}