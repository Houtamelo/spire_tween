use godot::classes::Control;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoControlAnchorBottom where Self: Control {
		#["anchor_bottom"]
		fn do_anchor_bottom(f64);
	}
}

impl_do_property! {
	pub trait DoControlAnchorLeft where Self: Control {
		#["anchor_left"]
		fn do_anchor_left(f64);
	}
}

impl_do_property! {
	pub trait DoControlAnchorRight where Self: Control {
		#["anchor_right"]
		fn do_anchor_right(f64);
	}
}

impl_do_property! {
	pub trait DoControlAnchorTop where Self: Control {
		#["anchor_top"]
		fn do_anchor_top(f64);
	}
}

impl_do_property! {
	pub trait DoControlMarginBottom where Self: Control {
		#["margin_bottom"]
		fn do_margin_bottom(f64);
	}
}

impl_do_property! {
	pub trait DoControlMarginLeft where Self: Control {
		#["margin_left"]
		fn do_margin_left(f64);
	}
}

impl_do_property! {
	pub trait DoControlMarginRight where Self: Control {
		#["margin_right"]
		fn do_margin_right(f64);
	}
}

impl_do_property! {
	pub trait DoControlMarginTop where Self: Control {
		#["margin_top"]
		fn do_margin_top(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectMinSize where Self: Control {
		#["rect_min_size"]
		fn do_rect_min_size(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectMinSizeX where Self: Control {
		#["rect_min_size:x"]
		fn do_rect_min_size_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectMinSizeY where Self: Control {
		#["rect_min_size:y"]
		fn do_rect_min_size_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectSize where Self: Control {
		#["rect_size"]
		fn do_rect_size(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectSizeX where Self: Control {
		#["rect_size:x"]
		fn do_rect_size_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectSizeY where Self: Control {
		#["rect_size:y"]
		fn do_rect_size_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectGlobalPosition where Self: Control {
		#["rect_global_position"]
		fn do_rect_global_position(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectGlobalPositionX where Self: Control {
		#["rect_global_position:x"]
		fn do_rect_global_position_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectGlobalPositionY where Self: Control {
		#["rect_global_position:y"]
		fn do_rect_global_position_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectPosition where Self: Control {
		#["rect_position"]
		fn do_rect_position(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectPositionX where Self: Control {
		#["rect_position:x"]
		fn do_rect_position_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectPositionY where Self: Control {
		#["rect_position:y"]
		fn do_rect_position_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectScale where Self: Control {
		#["rect_scale"]
		fn do_rect_scale(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectScaleX where Self: Control {
		#["rect_scale:x"]
		fn do_rect_scale_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectScaleY where Self: Control {
		#["rect_scale:y"]
		fn do_rect_scale_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectPivotOffset where Self: Control {
		#["rect_pivot_offset"]
		fn do_rect_pivot_offset(Vector2);
	}
}

impl_do_property! {
	pub trait DoControlRectPivotOffsetX where Self: Control {
		#["rect_pivot_offset:x"]
		fn do_rect_pivot_offset_x(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectPivotOffsetY where Self: Control {
		#["rect_pivot_offset:y"]
		fn do_rect_pivot_offset_y(f64);
	}
}

impl_do_property! {
	pub trait DoControlRectRotationDeg where Self: Control {
		#["rect_rotation"]
		fn do_rect_rotation_deg(f64);
	}
}