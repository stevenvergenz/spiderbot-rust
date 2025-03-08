
//! Ported from https://github.com/SpektrumRC/SRXL2/tree/master/Source by Steven Vergenz

//      7.1 General Overview
pub const SPEKTRUM_SRXL_ID: u8 = 0xA6;
pub const SRXL_MAX_BUFFER_SIZE: u8 = 80;
pub const SRXL_MAX_DEVICES: u8 = 16;

/// Supported SRXL device types (upper nibble of device ID)
pub enum SrxlDevType {
    None                = 0,
    RemoteReceiver      = 1,
    Receiver            = 2,
    FlightController    = 3,
    ESC                 = 4,
    SRXLServo1          = 6,
    SRXLServo2          = 7,
    VTX                 = 8,
    ExtRF               = 9,
    RemoteId            = 10,
    Sensor              = 11,
    Broadcast           = 15,
}

/// Default device ID list used by master when polling
pub const SRXL_DEFAULT_ID_OF_TYPE: [u8;16] = [
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

/// Set SRXL_CRC_OPTIMIZE_MODE in spm_srxl_config.h to one of the following values
pub mod crc_optimize_mode {
    /// Uses table lookup for CRC computation (requires 512 const bytes for CRC table)
    pub const SPEED: u8     = 1;
    /// Uses bitwise operations
    pub const SIZE: u8      = 2;
    /// Uses STM32 register-level hardware acceleration (only available on STM32F30x devices for now)
    pub const STM_HW: u8    = 3;
    /// Uses STM32Cube HAL driver for hardware acceleration (only available on STM32F3/F7) -- see srxlCrc16() for details on HAL config
    pub const STM_HAL: u8   = 4;
}

// Set SRXL_STM_TARGET_FAMILY in spm_srxl_config.h to one of the following values when using one of the STM HW-optimized modes
pub mod stm_target_family {
    /// STM32F3 family
    pub const F3: u8 = 3;
    /// STM32F7 family
    pub const F7: u8 = 7;
}

/// 7.2 Handshake Packet
pub mod handshake {
    pub const ID: u8 = 0x21;

    /// Supported additional baud rates besides default 115200
    /// NOTE: Treated as bitmask, ANDed with baud rates from slaves
    pub mod baud {
        pub const BAUD_115200: u8 = 0;
        pub const BAUD_400000: u8 = 1;
    }

    /// Bit masks for Device Info byte sent via Handshake
    pub mod dev_info {
        /// This is the base for non-RF devices
        pub const NO_RF: u8            = 0;
        /// This bit is set if the device is actively configured to transmit telemetry over RF
        pub const TELEM_TX_ENABLED: u8 = 1;
        /// This bit is set if the device can send full-range telemetry over RF
        pub const TELEM_FULL_RANGE: u8 = 2;
        /// This bit is set if the device supports Forward Programming via RF or SRXL
        pub const FWD_PROG_SUPPORT: u8 = 4;
    }
}

/// 7.3 Bind Info Packet
pub mod bind {
    pub const ID: u8             = 0x41;

    pub mod req {
        pub const ENTER: u8      = 0xEB;
        pub const STATUS: u8     = 0xB5;
        pub const BOUND_DATA: u8 = 0xDB;
        pub const SET_BIND: u8   = 0x5B;
    }

    /// Bit masks for Options byte
    pub mod opt {
        pub const NONE: u8            = 0x00;
        /// Set if this device should be enabled as the current telemetry device to tx over RF
        pub const TELEM_TX_ENABLE: u8 = 0x01;
        /// Set if this device should reply to a bind request with a Discover packet over RF
        pub const BIND_TX_ENABLE: u8  = 0x02;
        /// Set if this device should request US transmit power levels instead of EU
        pub const US_POWER: u8        = 0x04;
    }

    pub enum BindStatus {
        NotBound            = 0x00,
        // Air types
        Dsm2_1024_22ms      = 0x01,
        Dsm2_1024Mc24       = 0x02,
        Dsm2_2048_11ms      = 0x12,
        Dsmx22Ms            = 0xA2,
        Dsmx11Ms            = 0xB2,
        // Surface types (corresponding Air type bitwise OR'd with 0x40)
        SurfaceDsm1         = 0x40,
        SurfaceDsm2_16p6ms  = 0x63,
        Dsmr11ms22ms        = 0xE2,
        Dsmr5p5ms           = 0xE4,
    }

}

/// 7.4 Parameter Configuration
pub mod param {
    pub const ID: u8 = 0x50;
    pub mod req {
        pub const QUERY: u8 = 0x50;
        pub const WRITE: u8 = 0x57;
    }
}

/// 7.5 Signal Quality Packet
pub mod rssi {
    pub const ID: u8 = 0x55;
    pub mod req {
        pub const REQUEST: u8 = 0x52;
        pub const SEND: u8 = 0x53;
    }
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

pub mod vtx {
    pub mod band {
        pub const FATSHARK: u8 = 0;
        pub const RACEBAND: u8 = 1;
        pub const E_BAND: u8 = 2;
        pub const B_BAND: u8 = 3;
        pub const A_BAND: u8 = 4;
    }

    pub mod mode {
        pub const RACE: u8 = 0;
        pub const PIT: u8 = 1;
    }

    pub mod power {
        pub const OFF: u8 = 0;
        pub const P_1MW_14MW: u8 = 1;
        pub const P_15MW_25MW: u8 = 2;
        pub const P_26MW_99MW: u8 = 3;
        pub const P_100MW_299MW: u8 = 4;
        pub const P_300MW_600MW: u8 = 5;
        pub const P_601_PLUS: u8 = 6;
        pub const MANUAL: u8 = 7;
    }

    pub mod region {
        pub const US: u8 = 0;
        pub const EU: u8 = 1;
    }
}

pub const FWD_PGM_MAX_DATA_SIZE: u8 = 64;

/// Spektrum SRXL header
#[repr(packed)]
pub struct Header {
    /// Always 0xKA6 for SRXL2
    pub srxl_id: u8,
    pub packet_type: u8,
    pub length: u8,
}

/// Handshake
#[repr(packed)]
pub struct HandshakeData {
    pub src_dev_id: u8,
    pub dest_dev_id: u8,
    pub priority: u8,
    /// 0 = 115200, 1 = 400000 (See SRXL_BAUD_xxx definitions above)
    pub baud_supported: u8,
    /// See SRXL_DEVINFO_xxx definitions above for defined bits
    pub info: u8,
    /// Unique/random id to allow detection of two devices on bus with same deviceID
    pub uid: u32,
}

#[repr(packed)]
pub struct HandshakePacket {
    pub hdr: Header,
    pub payload: HandshakeData,
    pub crc: u16,
}

#[repr(packed)]
pub struct FullId {
    pub device_id: u8,
    pub bus_index: u8,
}
impl FullId {
    pub fn word(&self) -> u16 {
        ((self.device_id as u16) << 8) | (self.bus_index as u16)
    }
    pub fn set_word(&mut self, word: u16) {
        self.device_id = (word >> 8) as u8;
        self.bus_index = (word & 0xFF) as u8;
    }
}

#[repr(packed)]
pub struct VtxData {
    pub band: u8,
    pub channel: u8,
    pub pit: u8,
    pub power: u8,
    pub power_dec: u16,
    pub region: u8,
}
