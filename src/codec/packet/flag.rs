use libc::c_int;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
	pub struct Flags: c_int {
		const KEY     = AV_PKT_FLAG_KEY;
		const CORRUPT = AV_PKT_FLAG_CORRUPT;
	}
}
