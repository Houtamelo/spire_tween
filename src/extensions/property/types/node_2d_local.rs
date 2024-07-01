use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoNode2DMove where Self: Node2D {
		#["position"]
		fn do_move(Vector2);
	}
}

impl_do_property! {
	pub trait DoNode2DMoveX where Self: Node2D {
		#["position:x"]
		fn do_move_x(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DMoveY where Self: Node2D {
		#["position:y"]
		fn do_move_y(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DRotationDeg where Self: Node2D {
		#["rotation_degrees"]
		fn do_rotation(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DRotationRad where Self: Node2D {
		#["rotation"]
		fn do_rotation_rad(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DScale where Self: Node2D {
		#["scale"]
		fn do_scale(Vector2);
	}
}

impl_do_property! {
	pub trait DoNode2DScaleX where Self: Node2D {
		#["scale:x"]
		fn do_scale_x(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DScaleY where Self: Node2D {
		#["scale:y"]
		fn do_scale_y(f64);
	}
}