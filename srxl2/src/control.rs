use zerocopy::{
    CastError,
    TryCastError,
    KnownLayout,
    Immutable,
    FromBytes,
    TryFromBytes,
    IntoBytes,
};
use crate::{
    channel::ChannelData,
    device::DeviceId,
    fwd_pgm::FwdPgmData,
    packet::Header,
    vtx::VtxData
};

// header + cmd/replyID + crc
pub const BASE_LENGTH: u8 = 3 + 2 + 2;

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub enum CmdCode {
    Channel = 0x00,
    ChannelFailsafe = 0x01,
    Vtx = 0x02,
    FwdPgm = 0x03,
}

pub enum Cmd {
    None,
    Channel,
    ChannelFs,
    Vtx,
    FwdPgm,
    Rssi,
    Handshake,
    Telemtry,
    EnterBind,
    ReqBindInfo,
    SetBind,
    BindInfo,
    Internal,
}

const fn max2(a: usize, b: usize) -> usize {
    if b > a {
        b
    }
    else {
        a
    }
}

const fn max3(a: usize, b: usize, c: usize) -> usize {
    if c > a && c > b {
        c
    }
    else {
        max2(a, b)
    }
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct ControlData {
    pub cmd: CmdCode,
    pub reply_id: DeviceId,
    pub data: [u8; max3(
        size_of::<VtxData>(),
        size_of::<FwdPgmData>(),
        size_of::<ChannelData>(),
    )],
}

pub struct ControlPacket<'a> {
    pub hdr: &'a Header,
    pub control: &'a ControlData,
}

impl ControlData {
    /// Used for Channel Data and Failsafe Channel Data commands
    pub fn channel_data(&self) -> Result<&ChannelData, CastError<&[u8], ChannelData>> {
        ChannelData::ref_from_bytes(self.data.as_slice())
    }

    /// Used for VTX commands
    pub fn vtx_data(&self) -> Result<&VtxData, TryCastError<&[u8], VtxData>> {
        VtxData::try_ref_from_bytes(self.data.as_slice())
    }

    /// Used to pass forward programming data to an SRXL device
    pub fn fwd_pgm_data(&self) -> Result<&FwdPgmData, CastError<&[u8], FwdPgmData>> {
        FwdPgmData::ref_from_bytes(self.data.as_slice())
    }
}
