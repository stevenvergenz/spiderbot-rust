use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    IntoBytes,
};


pub const FWD_PGM_MAX_DATA_SIZE: usize = 64;

/// Forward Programming Data
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct FwdPgmData {
    /// Best RSSI while sending forward programming data
    pub rssi: i8,
    /// 0 for now -- used to word-align data
    pub rfu: [u8; 2],
    pub data: [u8; FWD_PGM_MAX_DATA_SIZE],
}
