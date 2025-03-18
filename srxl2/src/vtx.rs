use zerocopy::{
    KnownLayout,
    Immutable,
    TryFromBytes,
    IntoBytes,
};

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Band {
    FatShark = 0,
    RaceBand = 1,
    EBand = 2,
    BBand = 3,
    ABand = 4,
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Mode {
    Race = 0,
    Pit = 1,
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Power {
    Off = 0,
    P1To14Mw = 1,
    P15To99Mw = 2,
    P26To99Mw = 3,
    P100To299Mw = 4,
    P300To600Mw = 5,
    P601MwPlus = 6,
    Manual = 7,
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum Region {
    Us = 0,
    Eu = 1,
}

/// VTX Data
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct VtxData {
    pub band: Band,
    pub channel: u8,
    pub pit: Mode,
    pub power: Power,
    pub power_dec: u16,
    pub region: Region,
}
