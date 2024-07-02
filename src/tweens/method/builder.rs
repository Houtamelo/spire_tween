use crate::internal::*;

#[allow(private_bounds)]
impl<TVal: TweenableValue> SpireTween<Method<TVal>> {
	pub fn with_duration(self, duration: f64) -> Self {
		Self { 
			t: Method { duration, ..self.t },
			..self
		}
	}

	pub fn with_ease(self, ease: Ease) -> Self {
		Self {
			t: Method { ease, ..self.t },
			..self
		}
	}

	pub fn ending_at(self, end: TVal) -> Self {
		Self {
			t: Method { end, ..self.t },
			..self
		}
	}

	pub fn starting_at(self, start: TVal) -> Self {
		Self {
			t: Method { start, ..self.t },
			..self
		}
	}
}

#[allow(private_bounds)]
impl<TVal> SpireTween<Method<TVal>>
	where
		TVal: TweenableValue + SpireLerp,
		Method<TVal>: ValidTween,
{
	pub fn new(
		method: impl Into<StringName>,
		target: Gd<impl Inherits<Object>>,
		start: TVal,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
	) -> Self {
		Self {
			bound_node: None,
			state: match auto_play.0 {
				true => TweenState::Playing,
				false => TweenState::Paused,
			},
			delay: 0.,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			t: Method {
				method: method.into(),
				target: target.upcast(),
				duration,
				ease: Ease::Linear,
				start,
				end,
				lerp_fn: <TVal>::spire_lerp,
			},
			calls_on_finish: Vec::new(),
		}
	}

	pub fn new_registered(
		method: impl Into<StringName>,
		target: Gd<impl Inherits<Object>>,
		start: TVal,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
	) -> SpireHandle<Method<TVal>> {
		Self::new(method, target, start, end, duration, auto_play)
			.register()
	}
}

// Variant Builder
impl SpireTween<Method<Variant>> {
	pub fn new<TVal: SpireLerp>(
		method: impl Into<StringName>,
		target: Gd<impl Inherits<Object>>,
		start: TVal,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
	) -> Self {
		Self {
			bound_node: None,
			state: match auto_play.0 {
				true => TweenState::Playing,
				false => TweenState::Paused,
			},
			delay: 0.,
			speed_scale: 1.,
			elapsed_time: 0.,
			cycle_count: 0,
			pause_mode: TweenPauseMode::STOP,
			process_mode: TweenProcessMode::IDLE,
			loop_mode: LoopMode::Finite(0),
			t: Method {
				method: method.into(),
				target: target.upcast(),
				duration,
				ease: Ease::Linear,
				start: start.to_variant(),
				end: end.to_variant(),
				lerp_fn,
			},
			calls_on_finish: Vec::new(),
		}
	}

	pub fn new_registered<TVal: SpireLerp>(
		method: impl Into<StringName>,
		target: Gd<impl Inherits<Object>>,
		start: TVal,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
	) -> SpireHandle<Method<Variant>> {
		Self::new(method, target, start, end, duration, auto_play, lerp_fn)
			.register()
	}
}