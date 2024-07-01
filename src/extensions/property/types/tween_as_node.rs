use godot::classes::Tween;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoTweenPlaybackSpeed where Self: Tween {
		#["playback_speed"]
		fn do_playback_speed(f64);
	}
}