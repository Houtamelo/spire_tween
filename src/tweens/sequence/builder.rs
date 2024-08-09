use crate::internal::*;

impl Default for SpireTween<Sequence> {
	fn default() -> Self {
		Self::new()
	}
}

impl SpireTween<Sequence> {
	pub fn new() -> Self {
		Self {
			bound_node: None,
			state: TweenState::Playing,
			delay: 0.,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			calls_on_finish: Vec::new(),
			t: Sequence {
				queue: Vec::new(),
				inserteds: Vec::new(),
			}
		}
	}

	#[allow(private_bounds)]
	pub fn append(&mut self, tween: impl Into<AnyTween>) {
		let mut tween = tween.into();

		match self.state {
			| TweenState::Playing
			| TweenState::Paused => {
				tween.pause();
			}
			TweenState::Stopped => {
				tween.stop();
			}
		}

		self.t.queue.push(vec![tween.into()]);
	}

	pub fn append_call(&mut self, f: impl FnMut() + 'static) {
		let tween = SpireTween::<DelayedCall>::new(f, 0.0, AutoPlay(true));
		self.t.queue.push(vec![tween.into()]);
	}

	pub fn append_callable(&mut self, callable: Callable) {
		let tween = SpireTween::<DelayedCall>::new_callable(
			callable,
			0.0,
			AutoPlay(true),
		);
		
		self.t.queue.push(vec![tween.into()]);
	}

	pub fn append_interval(&mut self, time: f64) {
		let element = ForkElement::Interval { total_time: time, elapsed_time: 0. };
		self.t.queue.push(vec![element]);
	}

	#[allow(private_bounds)]
	pub fn join(&mut self, tween: impl Into<AnyTween>) {
		let mut tween = tween.into();

		match self.state {
			| TweenState::Playing
			| TweenState::Paused => {
				tween.pause();
			}
			TweenState::Stopped => {
				tween.stop();
			}
		}

		if let Some(back) = self.t.queue.last_mut() {
			back.push(tween.into());
		} else {
			self.append(tween);
		}
	}

	pub fn join_call(&mut self, f: impl FnMut() + 'static) {
		if let Some(back) = self.t.queue.last_mut() {
			let tween = SpireTween::<DelayedCall>::new(f, 0.0, AutoPlay(true));
			back.push(tween.into());
		} else {
			self.append_call(f);
		}
	}

	pub fn join_callable(&mut self, callable: Callable) {
		if let Some(back) = self.t.queue.last_mut() {
			let tween = SpireTween::<DelayedCall>::new_callable(
				callable, 
				0.0,
				AutoPlay(true),
			);
			
			back.push(tween.into());
		} else {
			self.append_callable(callable);
		}
	}

	#[allow(private_bounds)]
	pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) {
		let mut tween = tween.into();

		match self.state {
			| TweenState::Playing
			| TweenState::Paused => {
				tween.pause();
			}
			TweenState::Stopped => {
				tween.stop();
			}
		}

		self.t.inserteds.push((time, tween));
	}

	pub fn insert_call(&mut self, time: f64, f: impl FnMut() + 'static) {
		let tween = SpireTween::<DelayedCall>::new(f, 0.0, AutoPlay(true));
		self.t.inserteds.push((time, tween.into()));
	}
	
	pub fn insert_callable(&mut self, time: f64, callable: Callable) {
		let tween = SpireTween::<DelayedCall>::new_callable(
			callable, 
			0.0,
			AutoPlay(true),
		);
		
		self.t.inserteds.push((time, tween.into()));
	}
}