use zerocopy::{
    KnownLayout,
    Immutable,
    FromBytes,
    TryFromBytes,
    IntoBytes,
    CastError,
    TryCastError,
    ConvertError,
    ValidityError,
};
use crate::{
    handshake::HandshakeData,
    bind::BindPayload,
    param::ParamPayload,
    rssi::RssiPayload,
    telemetry::TelemetryPayload,
    control::{ControlData, ControlPacket},
};

pub const SRXL_MAX_BUFFER_SIZE: usize = 80;

/// Spektrum SRXL header
#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct Header {
    /// Always 0xKA6 for SRXL2
    pub srxl_id: u8,
    pub packet_type: PacketType,
    pub length: u8,
}

#[repr(u8)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
#[derive(PartialEq, Eq)]
pub enum PacketType {
    /// Handshake packet
    Handshake = 0x21,
    /// Bind info packet
    BindInfo = 0x41,
    /// Parameter config packet
    ParamConfig = 0x50,
    /// Signal quality packet
    SignalQuality = 0x55,
    /// Telemetry data packet
    TelemetryData = 0x80,
    /// Control data packet
    ControlData = 0xCD,
}

#[repr(C, packed)]
#[derive(KnownLayout, Immutable, TryFromBytes, IntoBytes)]
pub struct Packet {
    pub hdr: Header,
    pub raw: [u8; SRXL_MAX_BUFFER_SIZE - size_of::<Header>()],
}

pub struct BindPacket<'a> {
    pub hdr: &'a Header,
    pub bind: &'a BindPayload,
}

pub struct HandshakePacket<'a> {
    pub hdr: &'a Header,
    pub handshake: &'a HandshakeData,
}

pub struct ParamPacket<'a> {
    pub hdr: &'a Header,
    pub param: &'a ParamPayload,
}

pub struct RssiPacket<'a> {
    pub hdr: &'a Header,
    pub rssi: &'a RssiPayload,
}

pub struct TelemetryPacket<'a> {
    pub hdr: &'a Header,
    pub telemetry: &'a TelemetryPayload,
}

impl Packet {
    pub fn as_bind_ref(&self) -> Option<BindPacket> {
        if self.hdr.packet_type != PacketType::BindInfo {
            None
        }
        else {
            match BindPayload::try_ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(bind) => Some(BindPacket {
                    hdr: &self.hdr,
                    bind,
                }),
            }
        }
    }

    pub fn as_handshake_ref(&self) -> Option<HandshakePacket> {
        if self.hdr.packet_type != PacketType::Handshake {
            None
        }
        else {
            match HandshakeData::try_ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(handshake) => Some(HandshakePacket {
                    hdr: &self.hdr,
                    handshake,
                }),
            }
        }
    }

    pub fn as_param_ref(&self) -> Option<ParamPacket> {
        if self.hdr.packet_type != PacketType::ParamConfig {
            None
        }
        else {
            match ParamPayload::try_ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(param) => Some(ParamPacket {
                    hdr: &self.hdr,
                    param,
                }),
            }
        }
    }

    pub fn as_rssi_ref(&self) -> Option<RssiPacket> {
        if self.hdr.packet_type != PacketType::SignalQuality {
            None
        }
        else {
            match RssiPayload::try_ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(rssi) => Some(RssiPacket {
                    hdr: &self.hdr,
                    rssi,
                }),
            }
        }
    }

    pub fn as_telemetry_ref(&self) -> Option<TelemetryPacket> {
        if self.hdr.packet_type != PacketType::TelemetryData {
            None
        }
        else {
            match TelemetryPayload::ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(telemetry) => Some(TelemetryPacket {
                    hdr: &self.hdr,
                    telemetry,
                }),
            }
        }
    }

    pub fn as_control_ref(&self) -> Option<ControlPacket> {
        if self.hdr.packet_type != PacketType::ControlData {
            None
        }
        else {
            match ControlData::try_ref_from_bytes(self.raw.as_slice()) {
                Err(_) => None,
                Ok(control) => Some(ControlPacket {
                    hdr: &self.hdr,
                    control,
                }),
            }
        }
    }
}
