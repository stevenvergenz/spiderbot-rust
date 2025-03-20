
//! Ported from https://github.com/SpektrumRC/SRXL2/tree/master/Source by Steven Vergenz

use core::mem::size_of;
use zerocopy::{
    CastError, TryCastError, FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes,
};

//      7.1 General Overview
pub const SPEKTRUM_SRXL_ID: u8 = 0xA6;

// Set SRXL_STM_TARGET_FAMILY in spm_srxl_config.h to one of the following values when using one of the STM HW-optimized modes
#[repr(u8)]
pub enum StmTargetFamily {
    /// STM32F3 family
    F3 = 3,
    /// STM32F7 family
    F7 = 7,
}
