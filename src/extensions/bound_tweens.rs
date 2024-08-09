use crate::internal::*;

pub trait CompleteBoundTweens {
	fn complete_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> CompleteBoundTweens for Gd<T> {
	fn complete_bound_tweens(&mut self) {
		let node = self.clone().upcast();
		TweensController::map_mut(|brain| {
			brain.complete_boundeds(node);
		});
	}
}

pub trait KillBoundTweens {
	fn kill_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> KillBoundTweens for Gd<T> {
	fn kill_bound_tweens(&mut self) {
		let node = self.clone().upcast();
		TweensController::map_mut(|brain| {
			brain.kill_boundeds(node);
		});
	}
}
