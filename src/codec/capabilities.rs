use libc::c_uint;

use crate::ffi::*;

bitflags! {
	#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
	#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
	pub struct Capabilities: c_uint {
		const DRAW_HORIZ_BAND     = AV_CODEC_CAP_DRAW_HORIZ_BAND;
		const DR1                 = AV_CODEC_CAP_DR1;
		const TRUNCATED           = AV_CODEC_CAP_TRUNCATED;
		const DELAY               = AV_CODEC_CAP_DELAY;
		const SMALL_LAST_FRAME    = AV_CODEC_CAP_SMALL_LAST_FRAME;
		const SUBFRAMES           = AV_CODEC_CAP_SUBFRAMES;
		const EXPERIMENTAL        = AV_CODEC_CAP_EXPERIMENTAL;
		const CHANNEL_CONF        = AV_CODEC_CAP_CHANNEL_CONF;
		const FRAME_THREADS       = AV_CODEC_CAP_FRAME_THREADS;
		const SLICE_THREADS       = AV_CODEC_CAP_SLICE_THREADS;
		const PARAM_CHANGE        = AV_CODEC_CAP_PARAM_CHANGE;
		const AUTO_THREADS        = AV_CODEC_CAP_AUTO_THREADS;
		const VARIABLE_FRAME_SIZE = AV_CODEC_CAP_VARIABLE_FRAME_SIZE;
		#[cfg_attr(feature = "ffmpeg_4_3", deprecated)]
		const INTRA_ONLY          = AV_CODEC_CAP_INTRA_ONLY;
		#[cfg_attr(feature = "ffmpeg_4_3", deprecated)]
		const LOSSLESS            = AV_CODEC_CAP_LOSSLESS;
	}
}
