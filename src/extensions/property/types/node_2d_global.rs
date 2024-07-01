use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoNode2DGlobalMove where Self: Node2D {
		#["global_position"]
		fn do_global_move(Vector2);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalMoveX where Self: Node2D {
		#["global_position:x"]
		fn do_global_move_x(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalMoveY where Self: Node2D {
		#["global_position:y"]
		fn do_global_move_y(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalRotationDeg where Self: Node2D {
		#["global_rotation_degrees"]
		fn do_global_rotation(f64);
	}
}
	
impl_do_property! {
	pub trait DoNode2DGlobalRotationRad where Self: Node2D {
		#["global_rotation"]
		fn do_global_rotation_rad(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalScale where Self: Node2D {
		#["global_scale"]
		fn do_global_scale(Vector2);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalScaleX where Self: Node2D {
		#["global_scale:x"]
		fn do_global_scale_x(f64);
	}
}

impl_do_property! {
	pub trait DoNode2DGlobalScaleY where Self: Node2D {
		#["global_scale:y"]
		fn do_global_scale_y(f64);
	}
}
