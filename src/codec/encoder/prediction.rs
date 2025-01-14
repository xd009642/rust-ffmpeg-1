use libc::c_int;

use crate::ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Prediction {
	Left,
	Plane,
	Median,
}

impl From<c_int> for Prediction {
	fn from(value: c_int) -> Prediction {
		match value {
			FF_PRED_LEFT => Prediction::Left,
			FF_PRED_PLANE => Prediction::Plane,
			FF_PRED_MEDIAN => Prediction::Median,

			_ => Prediction::Left,
		}
	}
}

impl Into<c_int> for Prediction {
	fn into(self) -> c_int {
		match self {
			Prediction::Left => FF_PRED_LEFT,
			Prediction::Plane => FF_PRED_PLANE,
			Prediction::Median => FF_PRED_MEDIAN,
		}
	}
}
