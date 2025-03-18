use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    IntoBytes,
};
use crate::device::DeviceId;

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct TelemetryData {
    pub sensor_id: u8,
    pub secondary_id: u8,
    pub data: [u8; 14],
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct TelemetryPayload {
    pub dest_dev: DeviceId,
    pub payload: TelemetryData,
    pub crc: u16,
}