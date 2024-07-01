use godot::classes::CanvasModulate;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoCanvasModulateColor where Self: CanvasModulate {
		#["color"]
		fn do_canvas_color(Color);
	}
}

impl_do_property! {
	pub trait DoCanvasModulateColorR where Self: CanvasModulate {
		#["color:r"]
		fn do_canvas_color_r(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasModulateColorG where Self: CanvasModulate {
		#["color:g"]
		fn do_canvas_color_g(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasModulateColorB where Self: CanvasModulate {
		#["color:b"]
		fn do_canvas_color_b(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasModulateColorA where Self: CanvasModulate {
		#["color:a"]
		fn do_canvas_color_a(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasModulateFade where Self: CanvasModulate {
		#["color:a"]
		fn do_canvas_fade(f64);
	}
}
