use crate::{
    device::{DeviceEntry, DeviceId, FullId, MAX_DEVICES},
    flags::Flags,
    handshake::Baud,
    internal::State,
    packet::Packet,
    receiver::ReceiverEntry,
    tx::TxFlag,
};

#[cfg(feature = "1_bus")]
pub const NUM_OF_BUSES: usize = 1;

#[cfg(feature = "2_bus")]
pub const NUM_OF_BUSES: usize = 2;

#[cfg(feature = "4_bus")]
pub const NUM_OF_BUSES: usize = 4;

#[cfg(feature = "8_bus")]
pub const NUM_OF_BUSES: usize = 8;

pub const IS_HUB: bool = NUM_OF_BUSES > 1;

pub const ALL_BUSES: usize = (1 << NUM_OF_BUSES) - 1;

pub struct Bus {
    /// Transmit packet buffer
    srxl_out: Packet,
    /// Receive packet buffer
    srxl_in: Packet,
    /// Current state of SRXL state machine
    state: State,
    /// Device ID and Bus Index of this device, set during init
    full_id: FullId,
    /// Number of other SRXL devices discovered via handshake
    rx_dev_count: u8,
    /// Device entries for tracking SRXL telemetry priorities
    rx_dev: [DeviceEntry; MAX_DEVICES],
    /// Sum of priorities requested for each discovered SRXL device
    rx_dev_priority_sum: u16,
    /// Milliseconds since SRXL packet was received (incremented in srxlRun)
    timeout_count_ms: u16,
    /// Device ID to poll
    request_id: DeviceId,
    /// Baud rates this device can do: 0 = 115200, 1 = 400000
    baud_supported: Baud,
    /// Current baud rate: 0 = 115200, 1 = 400000
    baud_rate: Baud,
    /// Number of consecutive missed frames
    frame_err_count: u8,
    /// Pending outgoing packet types
    tx_flags: Flags<TxFlag>,
    /// Index number of UART tied to this SRXL bus
    uart: u8,
    /// Receiver entry for the bus master, if one exists
    master_rcvr: Option<ReceiverEntry>,
    /// Mask for channels to be sent on master buses
    channel_out_mask: u32,
    /// True if this device is the bus master on this bus
    master: bool,
    /// True if this bus master should poll all devices once more
    poll_once_more: bool,
    /// True when this SRXL bus is initialized
    initialized: bool,
}