use crate::internal::*;

impl<TVal: TweenableValue> SpireTweener for SpireTween<Method<TVal>> {
	fn state(&self) -> TweenState { self.state }

	fn set_state(&mut self, state: TweenState) {
		self.state = state;
	}
}

impl<TVal: TweenableValue> TweenerStep for SpireTween<Method<TVal>>
	where
		Method<TVal>: ValidTween,
{
	fn complete(mut self) {
		match self.state {
			| TweenState::Playing
			| TweenState::Paused => {
				self.seek_end();
			}
			TweenState::Stopped => {}
		}
	}

	fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
		self.elapsed_time += delta_time * self.speed_scale;

		if self.elapsed_time < self.delay {
			return None;
		}

		let target_value = {
			let eased_ratio = {
				let elapsed_ratio = ratio_with_delay_duration(
					self.delay,
					self.t.duration,
					self.elapsed_time,
				);

				self.t.ease.sample(elapsed_ratio)
			};

			(self.t.lerp_fn)(&self.t.start, &self.t.end, eased_ratio)
		};

		let maybe_excess_time = {
			let total_duration = self.delay + self.t.duration;
			let excess = self.elapsed_time - total_duration;
			(excess > 0.).then_some(excess)
		};

		self.t.target.call(self.t.method.clone(), &[target_value.to_variant()]);

		maybe_excess_time.and_then(|excess_time| {
			self.cycle_count += 1;

			match &mut self.loop_mode {
				LoopMode::Infinite => {
					self.elapsed_time = self.delay + excess_time;
					None
				}
				LoopMode::Finite(loop_count) => {
					if self.cycle_count < *loop_count {
						self.elapsed_time = self.delay + excess_time;
						None
					} else {
						self.elapsed_time -= excess_time;
						Some(excess_time)
					}
				}
			}
		}).inspect(|_| self.handle_finished())
	}
}

#[allow(private_bounds)]
impl<TVal> SpireTween<Method<TVal>>
	where
		Method<TVal>: ValidTween,
		TVal: TweenableValue,
{
	fn seek_end(&mut self) {
		if self.t.target.is_instance_valid() {
			let target_value = {
				let eased_ratio = self.t.ease.sample(1.);
				(self.t.lerp_fn)(&self.t.start, &self.t.end, eased_ratio)
			};

			self.t.target.call(
				self.t.method.clone(),
				&[target_value.to_variant()],
			);
		}

		self.handle_finished();
	}
}