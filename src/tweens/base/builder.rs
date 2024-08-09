use crate::internal::*;

#[allow(private_bounds)]
impl<T: ValidTween> SpireTween<T> {
	pub fn bound_to(
		self, 
		node: &impl ToGodot<Via = Gd<impl Inherits<Node>>>,
	) -> Self {
		Self {
			bound_node: Some(node.to_godot().upcast()),
			..self
		}
	}

	#[doc(hidden)]
	pub fn maybe_bound(
		self, 
		gd: &impl ToGodot<Via = Gd<impl Inherits<Object>>>,
	) -> Self {
		if let Ok(node) = gd.to_variant().try_to::<Gd<Node>>() {
			self.bound_to(&node)
		} else {
			self
		}
	}

	pub fn unbound(self) -> Self { Self { bound_node: None, ..self } }

	pub fn with_delay(self, delay: f64) -> Self { Self { delay, ..self } }

	pub fn with_speed_scale(self, speed_scale: f64) -> Self {
		Self { speed_scale, ..self }
	}

	pub fn with_pause_mode(self, pause_mode: TweenPauseMode) -> Self {
		Self { pause_mode, ..self }
	}

	pub fn with_process_mode(self, process_mode: TweenProcessMode) -> Self {
		Self { process_mode, ..self }
	}

	pub fn run_once(self) -> Self {
		Self {
			loop_mode: LoopMode::Finite(0),
			..self
		}
	}

	pub fn looped(self, loops: u32) -> Self {
		Self {
			loop_mode: LoopMode::Finite(loops),
			..self
		}
	}

	pub fn infinite(self) -> Self {
		Self {
			loop_mode: LoopMode::Infinite,
			..self
		}
	}

	pub fn on_finish(mut self, f: impl FnMut() + 'static) -> Self {
		self.calls_on_finish.push(f.into());
		self
	}

	pub fn on_finish_callable(mut self, callable: Callable) -> Self {
		self.calls_on_finish.push(callable.into());
		self
	}
}

#[allow(private_bounds)]
impl<T: ValidTween> SpireTween<T> {
	pub fn register(self) -> SpireHandle<T> {
		TweensController::map_mut(|brain| {
			brain.register(self)
		})
	}
}