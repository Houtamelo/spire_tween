use crate::internal::*;

impl SpireTweener for SpireTween<Sequence> {
	fn state(&self) -> State { self.state }

	fn set_state(&mut self, state: State) {
		match state {
			State::Playing => self.private_play(),
			State::Paused => self.private_pause(),
			State::Stopped => self.private_stop(),
		}
	}
}

impl SpireTween<Sequence> {
	fn private_play(&mut self) {
		if self.state == State::Playing {
			return;
		}

		let from_begin = self.state == State::Stopped;

		self.state = State::Playing;

		self.t.queue
		    .iter_mut()
		    .flat_map(|vec| vec.iter_mut())
		    .for_each(|fork_element| {
			    match fork_element {
				    ForkElement::Tween(tween) => {
					    if from_begin {
						    tween.stop();
						    tween.pause();
					    } else {
						    tween.pause();
					    }
				    }
				    ForkElement::Interval { elapsed_time, .. } => {
					    if from_begin {
						    *elapsed_time = 0.;
					    }
				    }
			    }
		    });

		self.t.inserteds
		    .iter_mut()
		    .for_each(|(_, tween)| {
			    if from_begin {
				    tween.stop();
				    tween.pause();
			    } else {
				    tween.pause();
			    }
		    });
	}

	fn private_pause(&mut self) {
		self.state = State::Paused;
	}

	fn private_stop(&mut self) {
		if self.state == State::Stopped {
			return;
		}

		self.state = State::Stopped;
		self.elapsed_time = 0.0;

		self.t.queue
		    .iter_mut()
		    .flat_map(|vec| vec.iter_mut())
		    .for_each(|fork_element| {
			    match fork_element {
				    ForkElement::Tween(tween) => {
					    tween.stop();
				    }
				    ForkElement::Interval { elapsed_time, .. } => {
					    *elapsed_time = 0.;
				    }
			    }
		    });

		self.t.inserteds
		    .iter_mut()
		    .for_each(|(_, tween)| tween.stop());
	}
}

impl TweenerStep for SpireTween<Sequence> {
	fn advance_time(&mut self, delta_time: f64) -> Option<f64> {
		let raw_delta_time = delta_time * self.speed_scale;
		let old_elapsed = self.elapsed_time;
		self.elapsed_time += delta_time;

		if self.elapsed_time < self.delay {
			return None;
		}

		let total_after_delay = self.elapsed_time - self.delay;

		let delta_time =
			if old_elapsed > self.delay {
				raw_delta_time
			} else {
				total_after_delay
			};

		for (at, tween) in self.t.inserteds.iter_mut() {
			match tween.state() {
				State::Playing => {
					tween.advance_time(delta_time);
				}
				State::Paused => {
					if *at <= total_after_delay {
						let above_at = total_after_delay - *at;
						tween.play();
						tween.advance_time(above_at);
					}
				}
				State::Stopped => {}
			}
		}

		let mut remaining_delta = delta_time;
		let mut queue_iter = self.t.queue.iter_mut();

		while let Some(fork) = queue_iter.next()
			&& remaining_delta > 0.
		{
			remaining_delta =
				fork.iter_mut()
				    .map(|fork_element| {
					    match fork_element {
						    ForkElement::Tween(tween) => {
							    match tween.state() {
								    State::Playing => {
									    tween.advance_time(remaining_delta)
								    }
								    State::Paused => {
									    tween.play();
									    tween.advance_time(remaining_delta)
								    }
								    State::Stopped => Some(remaining_delta),
							    }
						    }
						    ForkElement::Interval { total_time, elapsed_time } => {
							    *elapsed_time += remaining_delta;

							    let above_total = *elapsed_time - *total_time;
							    (above_total > 0.).then_some(above_total)
						    }
					    }
				    })
					.fold(Some(remaining_delta), |cur_min, time| {
						Some(f64::min(cur_min?, time?))
					}).unwrap_or(-1.);
		}

		if remaining_delta > 0. {
			self.handle_finished();
		}

		(remaining_delta > 0.).then_some(remaining_delta)
	}

	fn complete(mut self) {
		self.t.queue
		    .drain(..)
		    .for_each(|fork| {
			    fork.into_iter()
			        .for_each(|fork_element| {
				        match fork_element {
					        ForkElement::Tween(tween) => {
						        tween.complete();
					        }
					        ForkElement::Interval { .. } => {}
				        }
			        })
		    });

		self.t.inserteds
		    .drain(..)
		    .for_each(|(_, tween)| tween.complete());

		self.handle_finished();
	}
}