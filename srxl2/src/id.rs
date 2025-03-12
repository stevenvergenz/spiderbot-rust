use core::convert::TryFrom;
use zerocopy::{KnownLayout, Immutable, FromBytes, IntoBytes};

/// Default device ID list used by master when polling
const SRXL_DEFAULT_ID_OF_TYPE: [u8; 16] = [
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
        SRXL_DEFAULT_ID_OF_TYPE[self as usize]
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

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct DeviceId(u8);

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, FromBytes, IntoBytes)]
pub struct FullId {
    pub device_id: u8,
    pub bus_index: u8,
}
