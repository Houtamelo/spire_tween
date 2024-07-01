
use crate::internal::*;
use keyframe::EasingFunction;

#[derive(Debug, Clone)]
pub enum Ease {
	BezierCurve(keyframe::functions::BezierCurve),
	Keyframes(keyframe::functions::Keyframes),
	In,
	InCubic,
	InOut,
	InOutCubic,
	InOutQuad,
	InOutQuart,
	InOutQuint,
	InQuad,
	InQuart,
	InQuint,
	Out,
	OutCubic,
	OutQuad,
	OutQuart,
	OutQuint,
	Hold,
	Linear,
	Step,
}

impl Ease {
	pub fn sample(&self, x: f64) -> f64 {
		match self {
			Ease::BezierCurve(b) => b.y(x),
			Ease::Keyframes(k) => k.y(x),
			Ease::In => keyframe::functions::EaseIn.y(x),
			Ease::InCubic => keyframe::functions::EaseInCubic.y(x),
			Ease::InOut => keyframe::functions::EaseInOut.y(x),
			Ease::InOutCubic => keyframe::functions::EaseInOutCubic.y(x),
			Ease::InOutQuad => keyframe::functions::EaseInOutQuad.y(x),
			Ease::InOutQuart => keyframe::functions::EaseInOutQuart.y(x),
			Ease::InOutQuint => keyframe::functions::EaseInOutQuint.y(x),
			Ease::InQuad => keyframe::functions::EaseInQuad.y(x),
			Ease::InQuart => keyframe::functions::EaseInQuart.y(x),
			Ease::InQuint => keyframe::functions::EaseInQuint.y(x),
			Ease::Out => keyframe::functions::EaseOut.y(x),
			Ease::OutCubic => keyframe::functions::EaseOutCubic.y(x),
			Ease::OutQuad => keyframe::functions::EaseOutQuad.y(x),
			Ease::OutQuart => keyframe::functions::EaseOutQuart.y(x),
			Ease::OutQuint => keyframe::functions::EaseOutQuint.y(x),
			Ease::Hold => keyframe::functions::Hold.y(x),
			Ease::Linear => keyframe::functions::Linear.y(x),
			Ease::Step => keyframe::functions::Step.y(x),
		}
	}
}