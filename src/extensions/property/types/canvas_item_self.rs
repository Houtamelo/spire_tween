use godot::classes::CanvasItem;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoCanvasItemSelfColor where Self: CanvasItem {
		#["self_modulate"]
		fn do_self_color(Color);
	}
}

impl_do_property! {
	pub trait DoCanvasItemSelfColorR where Self: CanvasItem {
		#["self_modulate:r"]
		fn do_self_color_r(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemSelfColorG where Self: CanvasItem {
		#["self_modulate:g"]
		fn do_self_color_g(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemSelfColorB where Self: CanvasItem {
		#["self_modulate:b"]
		fn do_self_color_b(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemSelfColorA where Self: CanvasItem {
		#["self_modulate:a"]
		fn do_self_color_a(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemSelfFade where Self: CanvasItem {
		#["self_modulate:a"]
		fn do_self_fade(f64);
	}
}

// bellow just needs to compile
#[allow(unused)]
#[cfg(test)]
mod tests {
	use gdnative::api::Particles2D;

	use super::*;

	fn please_compile(node_direct: &Sprite, node_ref: Gd<CanvasItem>, node_tref: TGd<Particles2D>) {
		node_direct.do_self_color_r(1., 5.0);
		node_ref.do_self_color_r(1., 5.0);
		node_tref.do_self_color_r(1., 5.0);

		node_direct.do_self_color_g(1., 5.0);
		node_ref.do_self_color_g(1., 5.0);
		node_tref.do_self_color_g(1., 5.0);

		node_direct.do_self_color_b(1., 5.0);
		node_ref.do_self_color_b( 1., 5.0);
		node_tref.do_self_color_b( 1., 5.0);

		node_direct.do_self_color_a(1., 5.0);
		node_ref.do_self_color_a(1., 5.0);
		node_tref.do_self_color_a( 1., 5.0);

		node_direct.do_self_fade( 1., 5.0);
		node_ref.do_self_fade(1., 5.0);
		node_tref.do_self_fade( 1., 5.0);

		let color = Color::from_rgb(1., 1., 1.);
		node_direct.do_self_color(color, 5.0);
		node_ref.do_self_color(color, 5.0);
		node_tref.do_self_color(color, 5.0);
	}
}