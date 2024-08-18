use crate::internal::*;

impl<TVal: TweenableValue> SpireTweener for SpireTween<Property<TVal>> {
	fn state(&self) -> TweenState { self.state }
	fn set_state(&mut self, state: TweenState) {
		match state {
			TweenState::Stopped => {
				self.elapsed_time = 0.0;
				self.cycle_count = 0;
			}
			| TweenState::Playing
			| TweenState::Paused => {}
		}

		self.state = state;
	}
}

#[allow(private_bounds)]
impl<TVal: TweenableValue> SpireTween<Property<TVal>> {
	pub fn is_absolute(&self) -> bool {
		matches!(self.t.lerp_mode, LerpMode::Absolute { .. })
	}

	pub fn is_relative(&self) -> bool {
		matches!(self.t.lerp_mode, LerpMode::Relative { .. })
	}

	pub fn is_speed_based(&self) -> bool {
		matches!(self.t.lerp_mode, LerpMode::SpeedBased { .. })
	}
}

#[allow(private_bounds)]
impl<TVal: TweenableValue> SpireTween<Property<TVal>> {
	fn do_step(&mut self, delta_time: f64) -> Option<(TVal, Option<f64>)> {
		match &mut self.t.lerp_mode {
			LerpMode::Absolute { duration, start } => {
				let start_val = match &start {
					Some(val) => val,
					None => {
						let val_at_obj =
							match eval_property(&self.t.target, &self.t.property) {
								Ok(val) => val,
								Err(err) => {
									godot_error!("{err}");
									return None;
								}
							};

						start.replace(val_at_obj);
						start.as_ref().unwrap()
					}
				};

				let target_value = {
					let elapsed_ratio = ratio_with_delay_duration(
						self.delay,
						*duration,
						self.elapsed_time,
					);

					let eased_ratio = self.t.ease.sample(elapsed_ratio.min(1.));

					(self.t.lerp_fn)(
						start_val,
						&self.t.end,
						eased_ratio,
					)
				};

				let excess_time = {
					let total_duration = self.delay + *duration;
					let excess = self.elapsed_time - total_duration;
					if excess > 0. {
						Some(excess)
					} else {
						None
					}
				};

				Some((target_value, excess_time))
			}
			LerpMode::SpeedBased { speed, t_sum } => {
				let (target_value, step_result) = {
					let val_at_obj =
						match eval_property(&self.t.target, &self.t.property) {
							Ok(val) => val,
							Err(err) => {
								godot_error!("{err}");
								return None;
							}
						};

					(self.t.step_fn)(
						&val_at_obj,
						&self.t.end,
						*speed,
						delta_time + *t_sum,
					)
				};

				let excess_time =
					match step_result {
						StepResult::Unfinished { accumulated_t } => {
							*t_sum = accumulated_t;
							None
						}
						StepResult::Finished { excess_time } => {
							*t_sum = 0.;
							Some(excess_time)
						}
					};

				Some((target_value, excess_time))
			}
			LerpMode::Relative { duration, origin } => {
				let target_value = {
					let val_at_obj =
						match eval_property(&self.t.target, &self.t.property) {
							Ok(val) => val,
							Err(err) => {
								godot_error!("{err}");
								return None;
							}
						};

					let previous_eased_ratio = {
						let previous_ratio = ratio_with_delay_duration(
							self.delay,
							*duration,
							self.elapsed_time - delta_time,
						);

						self.t.ease.sample(previous_ratio)
					};

					let next_eased_ratio = {
						let elapsed_ratio = ratio_with_delay_duration(
							self.delay,
							*duration,
							self.elapsed_time,
						);

						self.t.ease.sample(elapsed_ratio)
					};

					let previous_relative = (self.t.lerp_fn)(
						origin,
						&self.t.end,
						previous_eased_ratio,
					);

					let next_relative = (self.t.lerp_fn)(
						origin,
						&self.t.end,
						next_eased_ratio,
					);

					(self.t.relative_fn)(
						&val_at_obj,
						&previous_relative,
						&next_relative,
					)
				};

				let excess_time = {
					let total_duration = self.delay + *duration;
					let excess = self.elapsed_time - total_duration;

					if excess > 0. {
						Some(excess)
					} else {
						None
					}
				};

				Some((target_value, excess_time))
			}
		}
	}
}

impl<TVal> TweenerStep for SpireTween<Property<TVal>>
	where
		TVal: TweenableValue,
		Property<TVal>: ValidTween,
{
	fn complete(mut self) {
		match self.state {
			| TweenState::Playing
			| TweenState::Paused => {
				match self.try_seek_end() {
					Ok(_) => {}
					Err(err) => { godot_error!("{err}"); }
				}

				self.handle_finished();
			}
			TweenState::Stopped => {}
		}
	}

	fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
		if !self.t.target.is_instance_valid() {
			self.stop();
			return None;
		}

		self.elapsed_time += delta_time * self.speed_scale;

		if self.elapsed_time < self.delay {
			return None;
		}

		let (target_value, maybe_excess_time) = self.do_step(delta_time)?;

		self.t.target.set_indexed(self.t.property.clone(), target_value.to_variant());

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
impl<TVal: TweenableValue> SpireTween<Property<TVal>> {
	fn try_seek_end(&mut self) -> anyhow::Result<()> {
		if !self.t.target.is_instance_valid() {
			let property = &self.t.property;
			bail!("Cannot set property `{property}` on Object, target does not point to a valid instance.");
		}

		let target_value =
			match &mut self.t.lerp_mode {
				| LerpMode::Absolute { .. }
				| LerpMode::SpeedBased { .. } => {
					self.t.end.clone()
				}
				LerpMode::Relative { duration, origin } => {
					let val_at_obj = eval_property(&self.t.target, &self.t.property)?;

					let previous_relative = {
						let previous_eased_ratio = {
							let previous_ratio = ratio_with_delay_duration(
								self.delay,
								*duration,
								self.elapsed_time,
							);

							self.t.ease.sample(previous_ratio)
						};

						(self.t.lerp_fn)(
							origin,
							&self.t.end,
							previous_eased_ratio,
						)
					};

					(self.t.relative_fn)(
						&val_at_obj,
						&previous_relative,
						&self.t.end,
					)
				}
			};

		self.t.target.set_indexed(
			self.t.property.clone(),
			target_value.to_variant(),
		);

		Ok(())
	}
}
