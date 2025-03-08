use core::todo;
use crate::types::*;

pub fn init_device(_device_id: u8, _priority: u8, _info: u8, _uid: u32) -> bool {
    todo!()
}

pub fn init_bus(_bus_index: u8, _uart: u8, _baud_supported: u8) -> bool {
    todo!()
}

pub fn is_bus_master(_bus_index: u8) -> bool {
    todo!()
}

pub fn get_timeout_count_ms(_bus_index: u8) -> u16 {
    todo!()
}

pub fn get_device_id(_bus_index: u8) -> u8 {
    todo!()
}

pub fn parse_packet(_bus_index: u8, _packet: &[u8]) -> bool {
    todo!()
}

pub fn run(_bus_index: u8, _timeout_delta_ms: i16) -> () {
    todo!()
}

pub fn enter_bind(_bind_type: u8, _broadcast: bool) -> bool {
    todo!()
}

pub fn set_bind_info(_bind_type: u8, _guid: u64, _uid: u32) -> bool {
    todo!()
}

pub fn request_bind_info(_bus_index: u8, _dest_dev_id: u8) -> bool {
    todo!()
}

pub fn on_frame_error(_bus_index: u8) -> () {
    todo!()
}

pub fn get_telemetry_endpoint() -> FullId {
    todo!()
}

pub fn set_vtx_data(_vtx_data: &VtxData) -> bool {
    todo!()
}

pub fn pass_thru_fwd_pgm(_data: &[u8]) -> bool {
    todo!()
}

pub fn set_hold_threshold(_countdown_reset: u8) -> () {
    todo!()
}

pub fn clear_comm_stats() -> () {
    todo!()
}

pub fn update_comm_stats(_is_fade: bool) -> bool {
    todo!()
}

pub fn send_internal_data(_bus_index: u8, _dest_dev_id: u8, _cmd: u8) -> () {
    todo!()
}
