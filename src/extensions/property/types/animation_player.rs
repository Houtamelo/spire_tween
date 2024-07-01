use godot::classes::AnimationPlayer;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoAnimationPlayerPlaybackSpeed where Self: AnimationPlayer {
		#["playback_speed"]
		fn do_playback_speed(f64);
	}
}