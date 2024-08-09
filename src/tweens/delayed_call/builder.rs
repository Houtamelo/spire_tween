use crate::internal::*;

impl SpireTween<DelayedCall> {
	pub fn new(
		f: impl FnMut() + 'static,
		delay: f64,
		auto_play: AutoPlay,
	) -> Self {
		Self {
			bound_node: None,
			state: match auto_play.0 {
				true => TweenState::Playing,
				false => TweenState::Paused,
			},
			delay,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			calls_on_finish: Vec::new(),
			t: f.into(),
		}
	}

	pub fn new_registered(
		f: impl FnMut() + 'static,
		delay: f64,
		auto_play: AutoPlay,
	) -> SpireHandle<DelayedCall> {
		Self::new(f, delay, auto_play)
			.register()
	}

	pub fn new_callable(
		callable: Callable,
		delay: f64,
		auto_play: AutoPlay,
	) -> Self {
		Self {
			bound_node: None,
			state: match auto_play.0 {
				true => TweenState::Playing,
				false => TweenState::Paused,
			},
			delay,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			calls_on_finish: Vec::new(),
			t: callable.into(),
		}
	}

	pub fn new_callable_registered(
		callable: Callable,
		delay: f64,
		auto_play: AutoPlay,
	) -> SpireHandle<DelayedCall> {
		Self::new_callable(callable, delay, auto_play)
			.register()
	}
}