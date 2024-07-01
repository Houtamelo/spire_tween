use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoAudioStreamPlayerVolumeDb where Self: AudioStreamPlayer {
		#["volume_db"]
		fn do_volume_db(f64);
	}
}

impl_do_property! {
	pub trait DoAudioStreamPlayerPitchScale where Self: AudioStreamPlayer {
		#["pitch_scale"]
		fn do_pitch_scale(f64);
	}
}