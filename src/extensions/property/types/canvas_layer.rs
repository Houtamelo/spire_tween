use godot::classes::CanvasLayer;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoCanvasLayerOffset where Self: CanvasLayer {
		#["offset"]
		fn do_offset(Vector2);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerOffsetX where Self: CanvasLayer {
		#["offset:x"]
		fn do_offset_x(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerOffsetY where Self: CanvasLayer {
		#["offset:y"]
		fn do_offset_y(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerRotationDegrees where Self: CanvasLayer {
		#["rotation_degrees"]
		fn do_rotation_degrees(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerRotationRadians where Self: CanvasLayer {
		#["rotation"]
		fn do_rotation_radians(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerScale where Self: CanvasLayer {
		#["scale"]
		fn do_scale(Vector2);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerScaleX where Self: CanvasLayer {
		#["scale:x"]
		fn do_scale_x(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerScaleY where Self: CanvasLayer {
		#["scale:y"]
		fn do_scale_y(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasLayerFollowViewportScale where Self: CanvasLayer {
		#["follow_viewport_scale"]
		fn do_follow_viewport_scale(f64);
	}
}