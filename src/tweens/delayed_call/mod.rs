use crate::internal::*;

pub mod methods;
pub mod builder;

pub enum DelayedCall {
	Callable(Callable),
	Closure(Box<dyn Fn()>),
}

impl DelayedCall {
	pub(crate) fn invoke(&self) {
		match self {
			DelayedCall::Callable(callable) => {
				callable.callv(VariantArray::new());
			}
			DelayedCall::Closure(closure) => {
				closure();
			}
		}
	}
}

impl<T: Fn() + 'static> From<T> for DelayedCall {
	fn from(value: T) -> Self {
		Self::Closure(Box::new(value))
	}
}

impl From<Callable> for DelayedCall {
	fn from(value: Callable) -> Self {
		Self::Callable(value)
	}
}

impl From<SpireTween<DelayedCall>> for AnyTween {
	fn from(value: SpireTween<DelayedCall>) -> Self {
		AnyTween::DelayedCall(Box::new(value))
	}
}

impl TweenConvert for DelayedCall {
	fn ref_from_any(tween: &mut AnyTween) -> Option<&mut SpireTween<Self>> {
		if let AnyTween::DelayedCall(t) = tween {
			Some(t)
		} else {
			None
		}
	}

	fn from_any(tween: AnyTween) -> Result<SpireTween<Self>, AnyTween> {
		if let AnyTween::DelayedCall(t) = tween {
			Ok(*t)
		} else {
			Err(tween)
		}
	}
}

