use crate::internal::*;
use crate::tweens::conversions::{delegate_impls, impl_from_variants};

pub mod base;
pub mod delayed_call;
pub mod method;
pub mod property;
pub mod lerping;
pub mod conversions;
pub mod sequence;
pub mod traits;
pub mod handle;
pub mod cfg;

pub(crate) enum AnyTween {
	Property(PropertyEnum),
	Method(MethodEnum),
	DelayedCall(Box<SpireTween<DelayedCall>>),
	Sequence(Box<SpireTween<Sequence>>),
}

pub(crate) fn ratio_with_delay_duration(delay: f64, duration: f64, elapsed_time: f64) -> f64 {
	f64::max((elapsed_time - delay) / duration, 0.)
}

delegate_impls! {
	pub enum AnyTween {
		Property(PropertyEnum),
		Method(MethodEnum),
		DelayedCall(Box<SpireTween<DelayedCall>>),
		Sequence(Box<SpireTween<Sequence>>),
	}
	
	pub trait SpireTweener {
		fn state(self: &Self,) -> State;
		fn set_state(self: &mut Self, state: State) -> ();
	}
}

delegate_impls! {
	pub enum AnyTween {
		Property(PropertyEnum),
		Method(MethodEnum),
		DelayedCall(Box<SpireTween<DelayedCall>>),
		Sequence(Box<SpireTween<Sequence>>),
	}
	
	pub trait TweenerStep {
		fn advance_time(self: &mut Self, delta_time: f64) -> Option<f64>;
		fn complete(self: Self,) -> ();
	}
}

delegate_impls! {
	pub enum AnyTween {
		Property(PropertyEnum),
		Method(MethodEnum),
		DelayedCall(Box<SpireTween<DelayedCall>>),
		Sequence(Box<SpireTween<Sequence>>),
	}
	
	pub trait Tick {
		fn tick_process(self: &mut Self, delta_time: f64) -> ();
		fn tick_physics(self: &mut Self, delta_time: f64) -> ();
		fn tick_independent(self: &mut Self, delta_time: f64) -> ();
	}
}

delegate_impls! {
	pub enum AnyTween {
		Property(PropertyEnum),
		Method(MethodEnum),
		DelayedCall(Box<SpireTween<DelayedCall>>),
		Sequence(Box<SpireTween<Sequence>>),
	}
	
	pub trait InnerTypeName {
		fn inner_type_name(self: &Self,) -> &'static str;
	}
}

impl_from_variants! {
	pub enum AnyTween {
		Property(PropertyEnum),
		Method(MethodEnum),
		DelayedCall(Box<SpireTween<DelayedCall>>),
		Sequence(Box<SpireTween<Sequence>>),
	}
}

impl AnyTween {
	pub(crate) fn bound_node(&self) -> &Option<Gd<Node>> {
		match self {
			AnyTween::Property(p) => {
				match p {
					PropertyEnum::i64(t) => &t.bound_node,
					PropertyEnum::f64(t) => &t.bound_node,
					PropertyEnum::String(t) => &t.bound_node,
					PropertyEnum::Color(t) => &t.bound_node,
					PropertyEnum::Vector2(t) => &t.bound_node,
					PropertyEnum::Vector3(t) => &t.bound_node,
					PropertyEnum::Variant(t) => &t.bound_node,
				}
			}
			AnyTween::Method(m) => {
				match m {
					MethodEnum::i64(t) => &t.bound_node,
					MethodEnum::f64(t) => &t.bound_node,
					MethodEnum::String(t) => &t.bound_node,
					MethodEnum::Color(t) => &t.bound_node,
					MethodEnum::Vector2(t) => &t.bound_node,
					MethodEnum::Vector3(t) => &t.bound_node,
					MethodEnum::Variant(t) => &t.bound_node,
				}
			}
			| AnyTween::DelayedCall(t) => &t.bound_node,
			| AnyTween::Sequence(t) => &t.bound_node,
		}
	} 
}

/*
pub trait Tick: Sized {

fn seek(&mut self, time: f64);

fn do_absolute_step(&mut self, delta: f64) {
	let current = self.elapsed_time();
	self.seek(current + delta);
}

fn do_scaled_step(&mut self, delta: f64) {
	let current = self.elapsed_time();
	self.seek(current + delta * self.speed_scale());
}

}
*/
