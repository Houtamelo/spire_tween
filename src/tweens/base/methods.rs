use crate::internal::*;
use crate::tweens::traits::InnerTypeName;

#[allow(private_bounds)]
impl<T> SpireTween<T>
	where
		SpireTween<T>: Tick,
		T: ValidTween,
{
	pub(crate) fn is_bounded_dead(&self) -> bool {
		self.bound_node.as_ref().is_some_and(|node| !node.is_instance_valid())
	}

	pub(crate) fn handle_finished(&mut self) {
		self.stop();
		self.calls_on_finish.iter_mut().for_each(DelayedCall::invoke);
	}
}

impl<T: ValidTween> Tick for SpireTween<T>
	where
		SpireTween<T>: TweenerStep,
{
	fn tick_process(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode != TweenProcessMode::IDLE {
			return;
		}

		// is_bounded_dead above already checks if the instance is valid
		let is_bounded_processing = self.bound_node.as_ref().is_some_and(|node| node.is_processing());

		if self.pause_mode == TweenPauseMode::BOUND
			&& !is_bounded_processing {
			return;
		}

		self.advance_time(delta_time);
	}

	fn tick_physics(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.process_mode != TweenProcessMode::PHYSICS {
			return;
		}

		// is_bounded_dead above already checks if the instance is valid
		let is_bounded_processing = self.bound_node.as_ref().is_some_and(|node| node.is_physics_processing());

		if self.pause_mode == TweenPauseMode::BOUND
			&& !is_bounded_processing {
			return;
		}

		self.advance_time(delta_time);
	}

	fn tick_independent(&mut self, delta_time: f64) {
		if self.is_bounded_dead() {
			self.stop();
			return;
		}

		if self.pause_mode != TweenPauseMode::PROCESS {
			return;
		}

		self.advance_time(delta_time);
	}
}

impl<T> InnerTypeName for SpireTween<T> {
	fn inner_type_name(&self) -> &'static str {
		type_name::<Self>()
	}
}