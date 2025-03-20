use zerocopy::{
    KnownLayout,
    Immutable,
    TryFromBytes,
    IntoBytes,
};
use crate::{
    device::{DeviceInfo, DeviceId}, bus::NUM_OF_BUSES
};

const MAX_RCVRS: usize = 2 * NUM_OF_BUSES;

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Received {
    None = 0,
    Dbm = 1,
    Pct = 2,
    Both = 3,
}

pub struct ReceiverEntry {
    /// SRXL device ID of the receiver
    pub device_id: DeviceId,
    /// Supports 8 buses, with each bit corresponding to busIndex (bit 0 = bus 0, bit 7 = bus 7)
    pub bus_bits: u8,
    /// Info bits reported during handshake - See SRXL_DEVINFO_XXX mask bits in header
    pub info: DeviceInfo,
    /// 0 = none, 1 = dBm, 2 = percent, 3 = both dBm and percent
    pub rssi_received: Received,
    /// Latest RSSI dBm value reported by receiver (negative, varies with receiver type)
    pub rssi_dbm: i8,
    /// Latest RSSI percent range estimate reported by receiver (0-100)
    pub rssi_pct: i8,
    /// Latest number of fades reported for a given receiver
    pub fades: u16,
    /// Latest channel mask for channels provided in channel data packet (0 during fade)
    pub channel_mask: u32,
}

pub struct ReceiverInfo {
    /// Stats for each receiver, filled when ch data is received
    pub rcvr_entry: [ReceiverEntry; MAX_RCVRS],
    rcvr_sorted_idx: [usize; MAX_RCVRS],
    /// Index into rcvrSorted where full-range telem rcvrs should be inserted
    pub rcvr_sort_insert: usize,
    /// Number of entries in rcvr[] and rcvrSorted[]
    pub rcvr_count: usize,
    pub rx_bus_bits: u8,
    pub best_rssi_dbm: i8,
    pub best_rssi_pct: i8,
    /// Reset to lossHoldCount when frame is good, and decrement for each consecutive frame loss
    /// -- when we get to 0, convert lossHoldCount frame losses to a hold
    pub loss_countdown: u8,
    /// Consecutive frame losses required to count as hold
    pub loss_hold_count: u8,
    /// Increment each time all receivers are in frame loss -- if 45
    /// consecutive, subtract those and increment holds
    pub frame_losses: u16,
    /// Increment each time 45 or more consecutive frames are lost (but don't keep
    /// incrementing once in that state)
    pub holds: u16,
    pub telem_rcvr_idx: usize,
    pub bind_rcvr_idx: usize,
}

impl<'a> ReceiverInfo {
    /// Pointers to receiver entries sorted in telemetry range order
    pub fn rcvr_sorted(&'a self) -> [&'a ReceiverEntry; MAX_RCVRS] {
        self.rcvr_sorted_idx.map(|i| &self.rcvr_entry[i])
    }

    /// Pointer to current assigned telemetry receiver (used for checking
    /// for fade to know when to switch)
    pub fn telem_rcvr(&'a self) -> &'a ReceiverEntry {
        &self.rcvr_entry[self.telem_rcvr_idx]
    }

    /// Pointer to receiver that we told to Enter Bind Mode (used to
    /// process Bound Data Report and send Set Bind Info)
    pub fn bind_rcvr(&'a self) -> &'a ReceiverEntry {
        &self.rcvr_entry[self.bind_rcvr_idx]
    }
}

pub type ReceiverStats = ReceiverInfo;
