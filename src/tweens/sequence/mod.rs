use crate::internal::*;

pub mod builder;
pub mod methods;
pub mod handle_impl;

pub struct Sequence {
	pub(crate) queue: Vec<Vec<ForkElement>>,
	pub(crate) inserteds: Vec<(f64, AnyTween)>,
}

impl From<SpireTween<Sequence>> for AnyTween {
	fn from(value: SpireTween<Sequence>) -> Self {
		AnyTween::Sequence(Box::new(value))
	}
}

impl TweenConvert for Sequence {
	fn ref_from_any(tween: &mut AnyTween) -> Option<&mut SpireTween<Self>> {
		if let AnyTween::Sequence(t) = tween {
			Some(t)
		} else {
			None
		}
	}

	fn from_any(tween: AnyTween) -> Result<SpireTween<Self>, AnyTween> {
		if let AnyTween::Sequence(t) = tween {
			Ok(*t)
		} else {
			Err(tween)
		}
	}
}

pub(crate) enum ForkElement {
	Tween(AnyTween),
	Interval { total_time: f64, elapsed_time: f64 },
}

impl<T: Into<AnyTween>> From<T> for ForkElement {
	fn from(value: T) -> Self {
		Self::Tween(value.into())
	}
}
