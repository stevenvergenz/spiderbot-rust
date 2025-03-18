use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    IntoBytes,
};


/// Channel Data
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ChannelData {
    /// Best RSSI when sending channel data, or dropout RSSI when sending failsafe data
    pub rssi: i8,
    /// Total lost frames (or fade count when sent from Remote Rx to main Receiver)
    pub frame_losses: u16,
    /// Set bits indicate that channel data with the corresponding index is present
    pub mask: u32,
    /// Channel values, shifted to full 16-bit range (32768 = mid-scale); lowest 2 bits RFU
    pub values: [u16; 32],
}

impl ChannelData {
    pub fn new() -> Self {
        Self {
            rssi: 0,
            frame_losses: 0,
            mask: 0,
            values: [0; 32],
        }
    }
}