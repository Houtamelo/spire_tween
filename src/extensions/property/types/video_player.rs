use godot::classes::VideoStreamPlayer;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoVideoPlayerVolumeDb where Self: VideoStreamPlayer {
		#["volume_db"]
		fn do_volume_db(f64);
	}
}

impl_do_property! {
	pub trait DoVideoPlayerVolume where Self: VideoStreamPlayer {
		#["volume"]
		fn do_volume(f64);
	}
}
