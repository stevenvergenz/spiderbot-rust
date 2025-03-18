#![no_std]

mod control;
mod channel;
mod bind;
mod device;
mod types;
mod fns;
mod packet;
mod receiver;
mod flags;
mod handshake;
mod internal;
mod param;
mod rssi;
mod telemetry;
mod vtx;
mod fwd_pgm;
mod error;

pub use types::*;
pub use fns::*;

#[cfg(feature = "1_bus")]
const NUM_OF_BUSES: usize = 1;

#[cfg(feature = "2_bus")]
const NUM_OF_BUSES: usize = 2;

#[cfg(feature = "4_bus")]
const NUM_OF_BUSES: usize = 4;

#[cfg(feature = "8_bus")]
const NUM_OF_BUSES: usize = 8;

const IS_HUB: bool = NUM_OF_BUSES > 1;

const ALL_BUSES: usize = (1 << NUM_OF_BUSES) - 1;