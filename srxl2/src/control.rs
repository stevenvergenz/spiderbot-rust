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
#[derive(PartialEq, Eq)]
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

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct ControlVtxData<'a> {
    pub cmd: &'a CmdCode,
    pub reply_id: &'a DeviceId,
    pub data: &'a VtxData,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct ControlFwdPgmData<'a> {
    pub cmd: &'a CmdCode,
    pub reply_id: &'a DeviceId,
    pub data: &'a FwdPgmData,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct ControlChannelData<'a> {
    pub cmd: &'a CmdCode,
    pub reply_id: &'a DeviceId,
    pub data: &'a ChannelData,
}

pub struct ControlPacket<'a> {
    pub hdr: &'a Header,
    pub control: &'a ControlData,
}

pub struct ControlVtxPacket<'a> {
    pub hdr: &'a Header,
    pub control: ControlVtxData<'a>,
}
pub struct ControlFwdPgmPacket<'a> {
    pub hdr: &'a Header,
    pub control: ControlFwdPgmData<'a>,
}
pub struct ControlChannelPacket<'a> {
    pub hdr: &'a Header,
    pub control: ControlChannelData<'a>,
}

impl<'a> ControlPacket<'a> {
    /// Used for Channel Data and Failsafe Channel Data commands
    pub fn as_channel_ref(&self) -> Option<ControlChannelPacket> {
        if self.control.cmd != CmdCode::Channel && self.control.cmd != CmdCode::ChannelFailsafe {
            None
        }
        else {
            match ChannelData::try_ref_from_bytes(self.control.data.as_slice()) {
                Err(_) => None,
                Ok(channel) => Some(ControlChannelPacket {
                    hdr: self.hdr,
                    control: ControlChannelData {
                        cmd: &self.control.cmd,
                        reply_id: &self.control.reply_id,
                        data: channel,
                    },
                }),
            }

        }
    }

    /// Used for VTX commands
    pub fn as_vtx_ref(&self) -> Option<ControlVtxPacket> {
        if self.control.cmd != CmdCode::Vtx {
            None
        }
        else {
            match VtxData::try_ref_from_bytes(self.control.data.as_slice()) {
                Err(_) => None,
                Ok(vtx) => Some(ControlVtxPacket {
                    hdr: self.hdr,
                    control: ControlVtxData {
                        cmd: &self.control.cmd,
                        reply_id: &self.control.reply_id,
                        data: vtx,
                    },
                }),
            }

        }
    }

    /// Used to pass forward programming data to an SRXL device
    pub fn as_fwd_pgm_ref(&self) -> Option<ControlFwdPgmPacket> {
        if self.control.cmd != CmdCode::FwdPgm {
            None
        }
        else {
            match FwdPgmData::try_ref_from_bytes(self.control.data.as_slice()) {
                Err(_) => None,
                Ok(fwd_pgm) => Some(ControlFwdPgmPacket {
                    hdr: self.hdr,
                    control: ControlFwdPgmData {
                        cmd: &self.control.cmd,
                        reply_id: &self.control.reply_id,
                        data: fwd_pgm,
                    },
                }),
            }

        }
    }
}
