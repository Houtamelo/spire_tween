use crate::internal::*;

impl SpireTweener for SpireTween<DelayedCall> {
	fn state(&self) -> State { self.state }

	fn set_state(&mut self, state: State) {
		self.state = state;
	}
}

impl TweenerStep for SpireTween<DelayedCall> {
	fn complete(mut self) {
		match self.state {
			| State::Playing
			| State::Paused => {
				self.seek_end();
			}
			State::Stopped => {}
		}
	}
	
	fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
		self.elapsed_time += delta_time * self.speed_scale;

		let excess = self.elapsed_time - self.delay;
		if excess <= 0. {
			return None;
		}

		self.t.invoke();

		self.cycle_count += 1;
		self.elapsed_time = excess;

		match &mut self.loop_mode {
			LoopMode::Infinite => {
				None
			}
			LoopMode::Finite(loop_count) => {
				if self.cycle_count < *loop_count {
					None
				} else {
					self.elapsed_time = self.delay;
					self.handle_finished();
					Some(excess)
				}
			}
		}
	}
}

impl SpireTween<DelayedCall> {
	fn seek_end(&mut self) {
		self.elapsed_time = self.delay;
		self.t.invoke();
		self.handle_finished();
	}
}