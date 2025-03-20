use core::convert::{From, TryFrom, Into};
use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    IntoBytes,
    TryFromBytes,
};

use crate::{
    flags::Flags,
    receiver::ReceiverEntry,
};

pub const MAX_DEVICES: usize = 16;

/// Default device ID list used by master when polling
const DEFAULT_ID_OF_TYPE: [u8; 16] = [
    0x00,  // SrxlDevType::None
    0x10,  // SrxlDevType::RemoteReceiver
    0x21,  // SrxlDevType::Receiver
    0x30,  // SrxlDevType::FlightController
    0x40,  // SrxlDevType::ESC
    0x60,  // 5
    0x60,  // SrxlDevType::SRXLServo1
    0x70,  // SrxlDevType::SRXLServo2
    0x81,  // SrxlDevType::VTX
    0x90,  // SrxlDevType::ExtRF
    0xA0,  // SrxlDevType::RemoteId
    0xB0,  // SrxlDevType::Sensor
    0xFF,  // 12
    0xFF,  // 13
    0xFF,  // 14
    0xFF,  // SrxlDevType::Broadcast
];

/// Supported SRXL device types (upper nibble of device ID)
#[repr(u8)]
pub enum DeviceType {
    None                = 0x0,
    RemoteReceiver      = 0x1,
    Receiver            = 0x2,
    FlightController    = 0x3,
    ESC                 = 0x4,
    SRXLServo1          = 0x6,
    SRXLServo2          = 0x7,
    VTX                 = 0x8,
    ExtRF               = 0x9,
    RemoteId            = 0xA,
    Sensor              = 0xB,
    Broadcast           = 0xF,
}

impl DeviceType {
    pub const fn default_value(self) -> u8 {
        DEFAULT_ID_OF_TYPE[self as usize]
    }
}

impl TryFrom<u8> for DeviceType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let nibble = value >> 4;
        if nibble == Self::None as u8 {
            Ok(Self::None)
        }
        else if nibble == Self::RemoteReceiver as u8 {
            Ok(Self::RemoteReceiver)
        }
        else if nibble == Self::Receiver as u8 {
            Ok(Self::Receiver)
        }
        else if nibble == Self::FlightController as u8 {
            Ok(Self::FlightController)
        }
        else if nibble == Self::ESC as u8 {
            Ok(Self::ESC)
        }
        else if nibble == Self::SRXLServo1 as u8 {
            Ok(Self::SRXLServo1)
        }
        else if nibble == Self::SRXLServo2 as u8 {
            Ok(Self::SRXLServo2)
        }
        else if nibble == Self::VTX as u8 {
            Ok(Self::VTX)
        }
        else if nibble == Self::ExtRF as u8 {
            Ok(Self::ExtRF)
        }
        else if nibble == Self::RemoteId as u8 {
            Ok(Self::RemoteId)
        }
        else if nibble == Self::Sensor as u8 {
            Ok(Self::Sensor)
        }
        else if nibble == Self::Broadcast as u8 {
            Ok(Self::Broadcast)
        }
        else {
            Err(())
        }
    }
}

/// Bit masks for Device Info byte sent via Handshake
#[repr(u8)]
#[derive(KnownLayout, Immutable, IntoBytes, TryFromBytes)]
pub enum DeviceInfo {
    /// This is the base for non-RF devices
    NoRf = 0x00,
    /// This bit is set if the device is actively configured to transmit telemetry over RF
    TelemTxEnabled = 0x01,
    /// This bit is set if the device can send full-range telemetry over RF
    TelemFullRange = 0x02,
    /// This bit is set if the device supports Forward Programming via RF or SRXL
    FwdProgSupport = 0x04,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct DeviceId(u8);

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct FullId {
    pub device_id: DeviceId,
    pub bus_index: u8,
}

pub struct DeviceEntry {
    pub device_id: DeviceId,
    /// Requested telemetry priority of this device
    pub priority: u8,
    /// Refer to SRXL_DEVINFO_XXX mask bits in header
    pub info: Flags<DeviceInfo>,
    pub rfu: u8,
}

pub struct Device {
    /// Device info for this local device, shared across all buses.
    pub dev_entry: DeviceEntry,
    /// ID statistically likely to be unique (Random, hash of serial, etc.)
    pub uid: u32,
    /// Pointer to our receiver entry, if we're a receiver (don't set for
    /// flight controller acting as hub -- only true receiver)
    pub rcvr: Option<ReceiverEntry>,
    /// Set true if this device can and should respond to VTX commands
    pub vtx_proxy: bool,
}