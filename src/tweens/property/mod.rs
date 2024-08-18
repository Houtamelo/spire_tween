use crate::internal::*;
use crate::tweens::conversions::{delegate_impls, impl_from_variants};

pub mod builder;
pub mod methods;
pub mod handle_impl;

pub struct Property<T> {
	pub property: NodePath,
	pub target: Gd<Object>,
	pub lerp_mode: LerpMode<T>,
	pub ease: Ease,
	pub end: T,
	lerp_fn: fn(from: &T, to: &T, f64) -> T,
	relative_fn: fn(value_at_obj: &T, previous_calc: &T, next_calc: &T) -> T,
	step_fn: fn(from: &T, to: &T, speed: f64, t: f64) -> (T, StepResult),
	distance_fn: fn(from: &T, to: &T) -> f64,
}

pub(crate) enum PropertyEnum {
	i64(Box<SpireTween<Property<i64>>>),
	f64(Box<SpireTween<Property<f64>>>),
	String(Box<SpireTween<Property<GString>>>),
	Color(Box<SpireTween<Property<Color>>>),
	Vector2(Box<SpireTween<Property<Vector2>>>),
	Vector3(Box<SpireTween<Property<Vector3>>>),
	Variant(Box<SpireTween<Property<Variant>>>),
}

pub(crate) fn eval_property<TVal: FromGodot, TObj: Inherits<Object>>(
	gd: &Gd<TObj>,
	property: &NodePath,
) -> anyhow::Result<TVal> {
	if !gd.is_instance_valid() {
		return Err(anyhow!("Cannot evaluate property, object does not point to a valid instance."));
	}
	
	let obj = gd.upcast_ref();
	
	let variant = obj.get_indexed(property.clone());

	variant
		.try_to::<TVal>()
		.map_err(|err| {
			let type_name = type_name::<TVal>();
			anyhow!("Target property `{property}` is not of type `{type_name}`, got: `{variant:?}`. \n\
				     Error: {err}")
		})
}

crate::tweens::conversions::impl_ref_from_any! {
	AnyTween::Property {
		Property<i64> => PropertyEnum::i64,
		Property<f64> => PropertyEnum::f64,
		Property<GString> => PropertyEnum::String,
		Property<Color> => PropertyEnum::Color,
		Property<Vector2> => PropertyEnum::Vector2,
		Property<Vector3> => PropertyEnum::Vector3,
		Property<Variant> => PropertyEnum::Variant,
	}
}

delegate_impls! {
	pub enum PropertyEnum {
		i64(Box<SpireTween<Property<i64>>>),
		f64(Box<SpireTween<Property<f64>>>),
		String(Box<SpireTween<Property<GString>>>),
		Color(Box<SpireTween<Property<Color>>>),
		Vector2(Box<SpireTween<Property<Vector2>>>),
		Vector3(Box<SpireTween<Property<Vector3>>>),
		Variant(Box<SpireTween<Property<Variant>>>),
	}
	
	pub trait SpireTweener {
		fn state(self: &Self,) -> TweenState;
		fn set_state(self: &mut Self, state: TweenState) -> ();
	}
}

delegate_impls! {
	pub enum PropertyEnum {
		i64(Box<SpireTween<Property<i64>>>),
		f64(Box<SpireTween<Property<f64>>>),
		String(Box<SpireTween<Property<GString>>>),
		Color(Box<SpireTween<Property<Color>>>),
		Vector2(Box<SpireTween<Property<Vector2>>>),
		Vector3(Box<SpireTween<Property<Vector3>>>),
		Variant(Box<SpireTween<Property<Variant>>>),
	}
	
	pub trait TweenerStep {
		fn advance_time(self: &mut Self, delta_time: f64) -> Option<f64>;
		fn complete(self: Self,) -> ();
	}
}

delegate_impls! {
	pub enum PropertyEnum {
		i64(Box<SpireTween<Property<i64>>>),
		f64(Box<SpireTween<Property<f64>>>),
		String(Box<SpireTween<Property<GString>>>),
		Color(Box<SpireTween<Property<Color>>>),
		Vector2(Box<SpireTween<Property<Vector2>>>),
		Vector3(Box<SpireTween<Property<Vector3>>>),
		Variant(Box<SpireTween<Property<Variant>>>),
	}
	
	pub trait Tick {
		fn tick_process(self: &mut Self, delta_time: f64) -> ();
		fn tick_physics(self: &mut Self, delta_time: f64) -> ();
		fn tick_independent(self: &mut Self, delta_time: f64) -> ();
	}
}

delegate_impls! {
	pub enum PropertyEnum {
		i64(Box<SpireTween<Property<i64>>>),
		f64(Box<SpireTween<Property<f64>>>),
		String(Box<SpireTween<Property<GString>>>),
		Color(Box<SpireTween<Property<Color>>>),
		Vector2(Box<SpireTween<Property<Vector2>>>),
		Vector3(Box<SpireTween<Property<Vector3>>>),
		Variant(Box<SpireTween<Property<Variant>>>),
	}
	
	pub trait InnerTypeName {
		fn inner_type_name(self: &Self,) -> &'static str;
	}
}

impl_from_variants! {
	pub enum PropertyEnum {
		i64(Box<SpireTween<Property<i64>>>),
		f64(Box<SpireTween<Property<f64>>>),
		String(Box<SpireTween<Property<GString>>>),
		Color(Box<SpireTween<Property<Color>>>),
		Vector2(Box<SpireTween<Property<Vector2>>>),
		Vector3(Box<SpireTween<Property<Vector3>>>),
		Variant(Box<SpireTween<Property<Variant>>>),
	}
}