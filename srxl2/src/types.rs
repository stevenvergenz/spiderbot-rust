
//! Ported from https://github.com/SpektrumRC/SRXL2/tree/master/Source by Steven Vergenz

use core::mem::size_of;
use zerocopy::{
    CastError, TryCastError, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes,
};

//      7.1 General Overview
pub const SPEKTRUM_SRXL_ID: u8 = 0xA6;
pub const SRXL_MAX_BUFFER_SIZE: usize = 80;
pub const SRXL_MAX_DEVICES: u8 = 16;

/// Set SRXL_CRC_OPTIMIZE_MODE in spm_srxl_config.h to one of the following values
#[repr(u8)]
pub enum CrcOptimizeMode {
    /// Uses table lookup for CRC computation (requires 512 const bytes for CRC table)
    Speed = 1,
    /// Uses bitwise operations
    Size = 2,
    /// Uses STM32 register-level hardware acceleration (only available on STM32F30x devices for now)
    StmHw = 3,
    /// Uses STM32Cube HAL driver for hardware acceleration (only available on STM32F3/F7) -- see srxlCrc16() for details on HAL config
    StmHal = 4,
}

// Set SRXL_STM_TARGET_FAMILY in spm_srxl_config.h to one of the following values when using one of the STM HW-optimized modes
#[repr(u8)]
pub enum StmTargetFamily {
    /// STM32F3 family
    F3 = 3,
    /// STM32F7 family
    F7 = 7,
}

/// 7.6 Telemetry Sensor Data Packet
pub const TELEM_ID: u8 = 0x80;

/// 7.7 Control Data Packet
pub mod ctrl {
    pub const ID: u8 = 0xCD;
    // header + cmd/replyID + crc
    pub const BASE_LENGTH: u8 = 3 + 2 + 2;

    pub mod cmd {
        pub const CHANNEL: u8 = 0x00;
        pub const CHANNEL_FS: u8 = 0x01;
        pub const VTX: u8 = 0x02;
        pub const FWDPGM: u8 = 0x03;
    }
}

/// X.X Spektrum Internal Use
pub const SPM_INTERNAL: u8 = 0x99;

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
