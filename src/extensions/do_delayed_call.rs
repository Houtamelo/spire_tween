use crate::internal::*;

pub trait DoDelayedCall {
	fn do_delayed_call(
		&self,
		call: impl Into<DelayedCall>,
		delay: f64,
	) -> SpireTween<DelayedCall>;
}

impl<T> DoDelayedCall for Gd<T>
	where
		T: GodotClass + Inherits<Object>
{
	fn do_delayed_call(
		&self,
		call: impl Into<DelayedCall>,
		delay: f64,
	) -> SpireTween<DelayedCall> {
		SpireTween::<DelayedCall>::new(call, delay, AutoPlay(true))
			.maybe_bound(self.clone())
	}
}
