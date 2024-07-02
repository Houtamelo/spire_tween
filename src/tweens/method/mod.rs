use crate::internal::*;
use crate::tweens::conversions::{delegate_impls, impl_from_variants};

pub mod builder;
pub mod methods;
pub mod handle_impl;

pub struct Method<T> {
	pub method: StringName,
	pub target: Gd<Object>,
	pub duration: f64,
	pub ease: Ease,
	pub start: T,
	pub end: T,
	lerp_fn: fn(from: &T, to: &T, f64) -> T,
}

pub(crate) enum MethodEnum {
	i64(Box<SpireTween<Method<i64>>>),
	f64(Box<SpireTween<Method<f64>>>),
	String(Box<SpireTween<Method<GString>>>),
	Color(Box<SpireTween<Method<Color>>>),
	Vector2(Box<SpireTween<Method<Vector2>>>),
	Vector3(Box<SpireTween<Method<Vector3>>>),
	Variant(Box<SpireTween<Method<Variant>>>),
}

crate::tweens::conversions::impl_ref_from_any! {
	AnyTween::Method {
		Method<i64> => MethodEnum::i64,
		Method<f64> => MethodEnum::f64,
		Method<GString> => MethodEnum::String,
		Method<Color> => MethodEnum::Color,
		Method<Vector2> => MethodEnum::Vector2,
		Method<Vector3> => MethodEnum::Vector3,
		Method<Variant> => MethodEnum::Variant,
	}
}

delegate_impls! {
	pub enum MethodEnum {
		i64(Box<SpireTween<Method<i64>>>),
		f64(Box<SpireTween<Method<f64>>>),
		String(Box<SpireTween<Method<GString>>>),
		Color(Box<SpireTween<Method<Color>>>),
		Vector2(Box<SpireTween<Method<Vector2>>>),
		Vector3(Box<SpireTween<Method<Vector3>>>),
		Variant(Box<SpireTween<Method<Variant>>>),
	}
	
	pub trait SpireTweener {
		fn state(self: &Self,) -> TweenState;
		fn set_state(self: &mut Self, state: TweenState) -> ();
	}
}

delegate_impls! {
	pub enum MethodEnum {
		i64(Box<SpireTween<Method<i64>>>),
		f64(Box<SpireTween<Method<f64>>>),
		String(Box<SpireTween<Method<GString>>>),
		Color(Box<SpireTween<Method<Color>>>),
		Vector2(Box<SpireTween<Method<Vector2>>>),
		Vector3(Box<SpireTween<Method<Vector3>>>),
		Variant(Box<SpireTween<Method<Variant>>>),
	}
	
	pub trait TweenerStep {
		fn advance_time(self: &mut Self, delta_time: f64) -> Option<f64>;
		fn complete(self: Self,) -> ();
	}
}

delegate_impls! {
	pub enum MethodEnum {
		i64(Box<SpireTween<Method<i64>>>),
		f64(Box<SpireTween<Method<f64>>>),
		String(Box<SpireTween<Method<GString>>>),
		Color(Box<SpireTween<Method<Color>>>),
		Vector2(Box<SpireTween<Method<Vector2>>>),
		Vector3(Box<SpireTween<Method<Vector3>>>),
		Variant(Box<SpireTween<Method<Variant>>>),
	}
	
	pub trait Tick {
		fn tick_process(self: &mut Self, delta_time: f64) -> ();
		fn tick_physics(self: &mut Self, delta_time: f64) -> ();
		fn tick_independent(self: &mut Self, delta_time: f64) -> ();
	}
}

delegate_impls! {
	pub enum MethodEnum {
		i64(Box<SpireTween<Method<i64>>>),
		f64(Box<SpireTween<Method<f64>>>),
		String(Box<SpireTween<Method<GString>>>),
		Color(Box<SpireTween<Method<Color>>>),
		Vector2(Box<SpireTween<Method<Vector2>>>),
		Vector3(Box<SpireTween<Method<Vector3>>>),
		Variant(Box<SpireTween<Method<Variant>>>),
	}
	
	pub trait InnerTypeName {
		fn inner_type_name(self: &Self,) -> &'static str;
	}
}

impl_from_variants! {
	pub enum MethodEnum {
		i64(Box<SpireTween<Method<i64>>>),
		f64(Box<SpireTween<Method<f64>>>),
		String(Box<SpireTween<Method<GString>>>),
		Color(Box<SpireTween<Method<Color>>>),
		Vector2(Box<SpireTween<Method<Vector2>>>),
		Vector3(Box<SpireTween<Method<Vector3>>>),
		Variant(Box<SpireTween<Method<Variant>>>),
	}
}