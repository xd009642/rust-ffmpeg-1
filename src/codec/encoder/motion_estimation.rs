use libc::c_int;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum MotionEstimation {
	Zero,
	Full,
	Log,
	Phods,
	Epzs,
	X1,
	Hex,
	Umh,
	Iter,
	Tesa,
}

impl From<c_int> for MotionEstimation {
	fn from(value: c_int) -> MotionEstimation {
		match value {
			1 => MotionEstimation::Zero,
			2 => MotionEstimation::Full,
			3 => MotionEstimation::Log,
			4 => MotionEstimation::Phods,
			5 => MotionEstimation::Epzs,
			6 => MotionEstimation::X1,
			7 => MotionEstimation::Hex,
			8 => MotionEstimation::Umh,
			9 => MotionEstimation::Iter,
			10 => MotionEstimation::Tesa,

			_ => MotionEstimation::Zero,
		}
	}
}

impl Into<c_int> for MotionEstimation {
	fn into(self) -> c_int {
		match self {
			MotionEstimation::Zero => 1,
			MotionEstimation::Full => 2,
			MotionEstimation::Log => 3,
			MotionEstimation::Phods => 4,
			MotionEstimation::Epzs => 5,
			MotionEstimation::X1 => 6,
			MotionEstimation::Hex => 7,
			MotionEstimation::Umh => 8,
			MotionEstimation::Iter => 9,
			MotionEstimation::Tesa => 10,
		}
	}
}
