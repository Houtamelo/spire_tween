use crate::internal::*;

impl SpireTween<DelayedCall> {
	pub fn new(
		call: impl Into<DelayedCall>,
		delay: f64,
		auto_play: AutoPlay,
	) -> Self {
		Self {
			bound_node: None,
			state: match auto_play.0 {
				true => State::Playing,
				false => State::Paused,
			},
			delay,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			calls_on_finish: Vec::new(),
			t: call.into(),
		}
	}

	pub fn new_registered(
		call: impl Into<DelayedCall>,
		delay: f64,
		auto_play: AutoPlay,
	) -> SpireHandle<DelayedCall> {
		Self::new(call, delay, auto_play)
			.register()
	}
}