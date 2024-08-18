use crate::internal::*;

pub trait CompleteBoundTweens {
	fn complete_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> CompleteBoundTweens for Gd<T> {
	fn complete_bound_tweens(&mut self) {
		if !self.is_instance_valid() {
			godot_warn!("Attempting to complete tweens bound to an Object that no longer exists.\n\
			             Any tweens that might have been bound were already stopped.");
			return;
		}
		
		let node = self.clone().upcast();
		TweensController::map_mut(|brain| {
			brain.tweens.extract_if(|_, tween| tween.bound_node().as_ref().is_some_and(|n| n == &node))
			    .map(|(_, tween)| tween)
			    .for_each(AnyTween::complete);
		});
	}
}

pub trait KillBoundTweens {
	fn kill_bound_tweens(&mut self);
}

impl<T: Inherits<Node>> KillBoundTweens for Gd<T> {
	fn kill_bound_tweens(&mut self) {
		if !self.is_instance_valid() {
			return;
		}
		
		let node = self.clone().upcast();
		TweensController::map_mut(|brain| {
			brain.tweens.retain(|_, tween| {
				tween.bound_node().as_ref().is_none_or(|n| {
					n.is_instance_valid() && n != &node
				})
			});
		});
	}
}
