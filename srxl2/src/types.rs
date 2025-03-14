
//! Ported from https://github.com/SpektrumRC/SRXL2/tree/master/Source by Steven Vergenz

use core::mem::size_of;
use zerocopy::{
    CastError, TryCastError, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes,
};

//      7.1 General Overview
pub const SPEKTRUM_SRXL_ID: u8 = 0xA6;
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
