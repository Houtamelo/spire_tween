use godot::classes::{Range, TextureProgressBar};
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoRangeValue where Self: Range {
		#["value"]
		fn do_value(f64);
	}
}

impl_do_property! {
	pub trait DoRangeRatio where Self: Range {
		#["ratio"]
		fn do_ratio(f64);
	}
}

impl_do_property! {
	pub trait DoTextureProgressTintUnder where Self: TextureProgressBar {
		#["tint_under"]
		fn do_tint_under(Color);
	}
}

impl_do_property! {
	pub trait DoTextureProgressTintOver where Self: TextureProgressBar {
		#["tint_over"]
		fn do_tint_over(Color);
	}
}

impl_do_property! {
	pub trait DoTextureProgressTintProgress where Self: TextureProgressBar {
		#["tint_progress"]
		fn do_tint_progress(Color);
	}
}

impl_do_property! {
	pub trait DoTextureProgressRadialInitialAngle where Self: TextureProgressBar {
		#["radial_initial_angle"]
		fn do_radial_initial_angle(f64);
	}
}

impl_do_property! {
	pub trait DoTextureProgressRadialFillDegrees where Self: TextureProgressBar {
		#["radial_fill_degrees"]
		fn do_radial_fill_degrees(f64);
	}
}

impl_do_property! {
	pub trait DoTextureProgressRadialCenterOffset where Self: TextureProgressBar {
		#["radial_center_offset"]
		fn do_radial_center_offset(Vector2);
	}
}

impl_do_property! {
	pub trait DoTextureProgressRadialCenterOffsetX where Self: TextureProgressBar {
		#["radial_center_offset:x"]
		fn do_radial_center_offset_x(f64);
	}
}

impl_do_property! {
	pub trait DoTextureProgressRadialCenterOffsetY where Self: TextureProgressBar {
		#["radial_center_offset:y"]
		fn do_radial_center_offset_y(f64);
	}
}

