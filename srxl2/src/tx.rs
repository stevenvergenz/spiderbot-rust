use zerocopy::{KnownLayout, Immutable, TryFromBytes, IntoBytes};

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum TxFlag {
    EnterBind = 0x01,
    GetBindInfo = 0x02,
    SetBindInfo = 0x04,
    BroadcastBindInfo = 0x08,
    ReportBindInfo = 0x10,
    SendVtxData = 0x20,
    SendFwdPgmData = 0x40,
    SendInternal = 0x80,
}