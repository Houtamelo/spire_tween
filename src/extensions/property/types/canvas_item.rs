use godot::classes::CanvasItem;
use crate::extensions::property::macros::impl_do_property;
use crate::internal::*;

impl_do_property! {
	pub trait DoCanvasItemColor where Self: CanvasItem {
		#["modulate"]
		fn do_color(Color);
	}
}

impl_do_property! {
	pub trait DoCanvasItemColorR where Self: CanvasItem {
		#["modulate:r"]
		fn do_color_r(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemColorG where Self: CanvasItem {
		#["modulate:g"]
		fn do_color_g(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemColorB where Self: CanvasItem {
		#["modulate:b"]
		fn do_color_b(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemColorA where Self: CanvasItem {
		#["modulate:a"]
		fn do_color_a(f64);
	}
}

impl_do_property! {
	pub trait DoCanvasItemFade where Self: CanvasItem {
		#["modulate:a"]
		fn do_fade(f64);
	}
}

// bellow just needs to compile
#[allow(unused)]
#[cfg(test)]
mod tests {
	use godot::prelude::*;
	use godot::classes::*;
	use super::*;

	fn test(node_direct: &Gd<Sprite2D>, node_ref: Gd<CanvasItem>, node_tref: Gd<CpuParticles2D>) {
		node_direct.do_color_r( 1., 5.);
		node_ref.do_color_r(1., 5.);
		node_tref.do_color_r(1., 5.);

		node_direct.do_color_g(1., 5.);
		node_ref.do_color_g(1., 5.);
		node_tref.do_color_g(1., 5.);

		node_direct.do_color_b(1., 5.);
		node_ref.do_color_b(1., 5.);
		node_tref.do_color_b(1., 5.);

		node_direct.do_color_a( 1., 5.);
		node_ref.do_color_a(1., 5.);
		node_tref.do_color_a(1., 5.);

		node_direct.do_fade(1., 5.);
		node_ref.do_fade(1., 5.);
		node_tref.do_fade(1., 5.);

		let color = Color::from_rgb(1., 1., 1.);
		node_direct.do_color(color, 5.);
		node_ref.do_color(color, 5.);
		node_tref.do_color(color, 5.);
	}

	#[derive(GodotClass)]
	#[class(init, base = Node2D)]
	struct Test {
		base: Base<Node2D>,
	}

	#[godot_api]
	impl INode2D for Test {
		fn ready(&mut self) {
			self.do_color_r(1., 2.0);
		}
	}
}