use std::libc::*;
use rust_util::c_bool;

pub struct ALLEGRO_MONITOR_INFO
{
	x1: c_int,
	y1: c_int,
	x2: c_int,
	y2: c_int,
}

pub static ALLEGRO_DEFAULT_DISPLAY_ADAPTER: i32 = -1;

extern "C"
{
	pub fn al_get_num_video_adapters() -> c_int;
	pub fn al_get_monitor_info(adapter: c_int, info: *ALLEGRO_MONITOR_INFO) -> c_bool;
}