use godot::classes::PathFollow2D;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoPathFollowOffset where Self: PathFollow2D {
		#["offset"]
		fn do_offset(f64);
	}
}

impl_do_property! {
	pub trait DoPathFollowUnitOffset where Self: PathFollow2D {
		#["unit_offset"]
		fn do_unit_offset(f64);
	}
}

impl_do_property! {
	pub trait DoPathFollowHOffset where Self: PathFollow2D {
		#["h_offset"]
		fn do_h_offset(f64);
	}
}

impl_do_property! {
	pub trait DoPathFollowVOffset where Self: PathFollow2D {
		#["v_offset"]
		fn do_v_offset(f64);
	}
}
