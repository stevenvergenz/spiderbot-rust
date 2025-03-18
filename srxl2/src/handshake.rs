use zerocopy::{KnownLayout, Immutable, TryFromBytes, IntoBytes};
use crate::{
    device::{DeviceInfo, DeviceId},
    flags::Flags,
};

/// Supported additional baud rates besides default 115200
/// NOTE: Treated as bitmask, ANDed with baud rates from slaves
#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Baud {
    Baud115200 = 0,
    Baud400000 = 1,
}

/// Handshake
#[repr(C,packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct HandshakeData {
    pub src_dev_id: DeviceId,
    pub dest_dev_id: DeviceId,
    pub priority: u8,
    /// 0 = 115200, 1 = 400000 (See SRXL_BAUD_xxx definitions above)
    pub baud_supported: Flags<Baud>,
    /// See SRXL_DEVINFO_xxx definitions above for defined bits
    pub info: Flags<DeviceInfo>,
    /// Unique/random id to allow detection of two devices on bus with same deviceID
    pub uid: u32,
}
