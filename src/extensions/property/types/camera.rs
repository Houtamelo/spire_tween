use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoCameraZoom where Self: Camera2D {
		#["zoom"]
		fn do_zoom(Vector2);
	}
}

impl_do_property! {
	pub trait DoCameraZoomX where Self: Camera2D {
		#["zoom:x"]
		fn do_zoom_x(f64);
	}
}

impl_do_property! {
	pub trait DoCameraZoomY where Self: Camera2D {
		#["zoom:y"]
		fn do_zoom_y(f64);
	}
}

impl_do_property! {
	pub trait DoCameraOffset where Self: Camera2D {
		#["offset"]
		fn do_offset(Vector2);
	}
}

impl_do_property! {
	pub trait DoCameraOffsetX where Self: Camera2D {
		#["offset:x"]
		fn do_offset_x(f64);
	}
}

impl_do_property! {
	pub trait DoCameraOffsetY where Self: Camera2D {
		#["offset:y"]
		fn do_offset_y(f64);
	}
}

impl_do_property! {
	pub trait DoCameraOffsetH where Self: Camera2D {
		#["offset_h"]
		fn do_offset_h(f64);
	}
}

impl_do_property! {
	pub trait DoCameraOffsetV where Self: Camera2D {
		#["offset_v"]
		fn do_offset_v(f64);
	}
}

impl_do_property! {
	pub trait DoCameraDragMarginLeft where Self: Camera2D {
		#["drag_margin_left"]
		fn do_drag_margin_left(f64);
	}
}

impl_do_property! {
	pub trait DoCameraDragMarginRight where Self: Camera2D {
		#["drag_margin_right"]
		fn do_drag_margin_right(f64);
	}
}

impl_do_property! {
	pub trait DoCameraDragMarginTop where Self: Camera2D {
		#["drag_margin_top"]
		fn do_drag_margin_top(f64);
	}
}

impl_do_property! {
	pub trait DoCameraDragMarginBottom where Self: Camera2D {
		#["drag_margin_bottom"]
		fn do_drag_margin_bottom(f64);
	}
}
