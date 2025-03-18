use zerocopy::{KnownLayout, Immutable, FromBytes, IntoBytes};
use crate::device::DeviceId;

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct InternalData {
    pub src_dev_id: DeviceId,
    pub dest_dev_id: DeviceId,
    pub test: u8,
    pub key: u32,
}

pub enum State {
    /// Default state before initialized or if bus is subsequently disabled
    Disabled,
    /// Wait 50ms to see if anything is already talking (i.e. we probably browned out)
    ListenOnStartup,
    /// Call when handshake should be sent every 50ms
    SendHandshake,
    /// Wait at least 150ms more for handshake request
    ListenForHandshake,
    /// Normal run state
    Running,
    /// Send telemetry reply when requested
    SendTelemetry,
    /// Send VTX packet when needed
    SendVtx,
    SendEnterBind,
    SendBoundDataReport,
    SendSetBindInfo,
    RequestBindInfo,
    SendInternal,
}