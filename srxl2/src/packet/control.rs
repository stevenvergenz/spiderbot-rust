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
    packet::{
        Header,
        channel::ChannelData,
        fwd_pgm::FwdPgmData,
        vtx::VtxData,
    },
    id::DeviceId,
};

// header + cmd/replyID + crc
pub const BASE_LENGTH: u8 = 3 + 2 + 2;

pub mod cmd {
    pub const CHANNEL: u8 = 0x00;
    pub const CHANNEL_FS: u8 = 0x01;
    pub const VTX: u8 = 0x02;
    pub const FWDPGM: u8 = 0x03;
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

const fn max(a: usize, b: usize, c: usize) -> usize {
    if b > a {
        if c > b {
            c
        }
        else {
            b
        }
    }
    else {
        if c > a {
            c
        }
        else {
            a
        }
    }
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ControlData {
    pub cmd: u8,
    pub reply_id: u8,
    pub data: [u8; max(
        size_of::<VtxData>(),
        size_of::<FwdPgmData>(),
        size_of::<ChannelData>(),
    )],
}

impl ControlData {
    pub fn channel_data(&self) -> Result<&ChannelData, CastError<&[u8], ChannelData>> {
        ChannelData::ref_from_bytes(self.data.as_slice())
    }

    pub fn vtx_data(&self) -> Result<&VtxData, TryCastError<&[u8], VtxData>> {
        VtxData::try_ref_from_bytes(self.data.as_slice())
    }

    pub fn fwd_pgm_data(&self) -> Result<&FwdPgmData, CastError<&[u8], FwdPgmData>> {
        FwdPgmData::ref_from_bytes(self.data.as_slice())
    }
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct ControlPacket {
    pub hdr: Header,
    pub payload: ControlData,
}
