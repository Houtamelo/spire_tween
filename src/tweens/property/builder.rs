use crate::internal::*;

#[allow(private_bounds)]
impl<TVal: TweenableValue> SpireTween<Property<TVal>> {
	pub fn with_duration(self, duration: f64) -> Self {
		match self.t.lerp_mode {
			LerpMode::Absolute { start, .. } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Absolute { duration, start },
						..self.t
					},
					..self
				}
			}
			LerpMode::Relative { origin, .. } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Relative { duration, origin },
						..self.t
					},
					..self
				}
			}
			LerpMode::SpeedBased { .. } => {
				godot_warn!(
					"Duration is not used in current lerp mode (SpeedBased)."
				);
				self
			}
		}
	}

	pub fn with_ease(self, ease: Ease) -> Self {
		Self {
			t: Property { ease, ..self.t },
			..self
		}
	}

	pub fn ending_at(self, value: TVal) -> Self {
		Self {
			t: Property { end: value, ..self.t },
			..self
		}
	}

	pub fn starting_at(self, value: TVal) -> Self {
		match self.t.lerp_mode {
			LerpMode::Absolute { duration, .. } =>
				Self {
					t: Property {
						lerp_mode: LerpMode::Absolute {
							duration,
							start: Some(value),
						},
						..self.t
					},
					..self
				},
			LerpMode::SpeedBased { .. } => {
				godot_warn!(
					"Starting value is not used in current lerp mode \
					 (SpeedBased)."
				);
				self
			}
			LerpMode::Relative { duration, .. } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Relative { duration, origin: value },
						..self.t
					},
					..self
				}
			}
		}
	}

	pub fn evaluate_start_on_play(self) -> Self {
		if let LerpMode::Absolute { duration, .. } = self.t.lerp_mode {
			Self {
				t: Property {
					lerp_mode: LerpMode::Absolute { duration, start: None },
					..self.t
				},
				..self
			}
		} else {
			godot_warn!(
				"Evaluating start value on play is only used in `Absolute` \
				 lerp mode."
			);
			self
		}
	}

	pub fn as_absolute(self) -> Self
		where
			TVal: SpireLerp,
	{
		match self.t.lerp_mode {
			LerpMode::SpeedBased { speed, .. } => {
				let eval_result = eval_property(
					self.t.target.clone(),
					self.t.property.clone(),
				);

				let val_at_obj =
					match eval_result {
						Ok(val) => val,
						Err(err) => {
							godot_error!("{err}");
							return self;
						}
					};

				let distance = (self.t.distance_fn)(&val_at_obj, &self.t.end);
				let duration = distance / speed;

				Self {
					t: Property {
						lerp_mode: LerpMode::Absolute { duration, start: None },
						..self.t
					},
					..self
				}
			}
			LerpMode::Relative { duration, origin } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Absolute {
							duration,
							start: Some(origin),
						},
						..self.t
					},
					..self
				}
			}
			LerpMode::Absolute { .. } => self,
		}
	}

	pub fn as_speed_based(self, speed: f64) -> Self {
		Self {
			t: Property {
				lerp_mode: LerpMode::SpeedBased { speed, t_sum: 0. },
				..self.t
			},
			..self
		}
	}

	pub fn as_relative(self, origin: TVal) -> Self {
		match self.t.lerp_mode {
			LerpMode::Absolute { duration, .. } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Relative { duration, origin },
						..self.t
					},
					..self
				}
			}
			LerpMode::SpeedBased { speed, .. } => {
				let distance = (self.t.distance_fn)(&origin, &self.t.end);
				let duration = distance / speed;

				Self {
					t: Property {
						lerp_mode: LerpMode::Relative { duration, origin },
						..self.t
					},
					..self
				}
			}
			LerpMode::Relative { duration, .. } => {
				Self {
					t: Property {
						lerp_mode: LerpMode::Relative { duration, origin },
						..self.t
					},
					..self
				}
			}
		}
	}
}

#[allow(private_bounds)]
impl<TVal> SpireTween<Property<TVal>>
	where
		Property<TVal>: ValidTween,
		TVal: TweenableValue + SpireLerp,
{
	pub fn new(
		property: impl Into<NodePath>,
		target: &impl ToGodot<Via = Gd<impl Inherits<Object>>>,
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
			calls_on_finish: Vec::new(),
			t: Property {
				property: property.into(),
				target: target.to_godot().upcast(),
				lerp_mode: LerpMode::Absolute { duration, start: None },
				ease: Ease::Linear,
				end,
				lerp_fn: <TVal>::spire_lerp,
				relative_fn: <TVal>::add_relative,
				step_fn: <TVal>::spire_step,
				distance_fn: <TVal>::spire_distance,
			},
		}
	}

	pub fn new_registered(
		property: impl Into<NodePath>,
		target: &impl ToGodot<Via = Gd<impl Inherits<Object>>>,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
	) -> SpireHandle<Property<TVal>> {
		Self::new(property, target, end, duration, auto_play)
			.register()
	}
}

// Variant Builder
impl SpireTween<Property<Variant>> {
	pub fn new<TVal: SpireLerp>(
		property: impl Into<NodePath>,
		target: &impl ToGodot<Via = Gd<impl Inherits<Object>>>,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
		step_fn: fn(from: &Variant, to: &Variant, speed: f64, t: f64) -> (Variant, StepResult),
		distance_fn: fn(from: &Variant, to: &Variant) -> f64,
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
			calls_on_finish: Vec::new(),
			t: Property {
				property: property.into(),
				target: target.to_godot().upcast(),
				lerp_mode: LerpMode::Absolute { duration, start: None },
				ease: Ease::Linear,
				end: end.to_variant(),
				lerp_fn,
				relative_fn,
				step_fn,
				distance_fn,
			},
		}
	}

	pub fn new_registered<TVal: SpireLerp>(
		property: impl Into<NodePath>,
		target: &impl ToGodot<Via = Gd<impl Inherits<Object>>>,
		end: TVal,
		duration: f64,
		auto_play: AutoPlay,
		lerp_fn: fn(from: &Variant, to: &Variant, f64) -> Variant,
		relative_fn: fn(value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant) -> Variant,
		step_fn: fn(from: &Variant, to: &Variant, speed: f64, t: f64) -> (Variant, StepResult),
		distance_fn: fn(from: &Variant, to: &Variant) -> f64,
	) -> SpireHandle<Property<Variant>> {
		Self::new(property, target, end, duration, auto_play, lerp_fn, relative_fn, step_fn, distance_fn)
			.register()
	}
}
