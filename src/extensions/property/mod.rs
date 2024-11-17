use godot::obj::WithBaseField;
use crate::internal::*;

pub mod macros;
pub mod types;

pub trait DoProperty<TVal, Marker = ()> {
	fn do_property(
		&self,
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<TVal>>;
}

impl<T, TVal> DoProperty<TVal, ()> for Gd<T>
	where
		T: Inherits<Object>,
		TVal: TweenableValue + SpireLerp,
		Property<TVal>: ValidTween,
{
	fn do_property(
		&self,
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<TVal>> {
		SpireTween::<Property<TVal>>::new(
			property,
			self,
			end_val,
			duration,
			AutoPlay(true),
		).maybe_bound(self)
	}
}

impl<TVal, T> DoProperty<TVal, BaseMarker> for T
	where
		T: WithBaseField + Inherits<Object>,
		TVal: TweenableValue + SpireLerp,
		Property<TVal>: ValidTween,
{
	fn do_property(
		&self,
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<TVal>> {
		SpireTween::<Property<TVal>>::new(
			property,
			&self.to_gd(),
			end_val,
			duration,
			AutoPlay(true),
		).maybe_bound(&self.to_gd())
	}
}

pub trait DoVarProperty<TVal: ToGodot, Marker = ()> {
	fn do_var_property(
		&self,
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<Variant>>;
}

impl<T, TVal> DoVarProperty<TVal, ()> for Gd<T>
	where
		T: Inherits<Object>,
		TVal: Sized + Clone + ToGodot + FromGodot + SpireLerp, 
{
	fn do_var_property(
		&self, 
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<Variant>> {
		let lerp_fn = |from: &Variant, to: &Variant, t: f64| -> Variant {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();
			
			TVal::spire_lerp(&from, &to, t).to_variant()
		};
		
		let relative_fn = |value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant| -> Variant {
			let value_at_obj = value_at_obj.to::<TVal>();
			let previous_calc = previous_calc.to::<TVal>();
			let next_calc = next_calc.to::<TVal>();
			
			TVal::add_relative(&value_at_obj, &previous_calc, &next_calc).to_variant()
		};
		
		let step_fn = |from: &Variant, to: &Variant, speed: f64, t: f64| -> (Variant, StepResult) {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();
			
			let (step, result) = TVal::spire_step(&from, &to, speed, t);
			(step.to_variant(), result)
		};
		
		let distance_fn = |from: &Variant, to: &Variant| -> f64 {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();
			
			TVal::spire_distance(&from, &to)
		};
		
		SpireTween::<Property<Variant>>::new::<TVal>(
			property,
			self,
			end_val,
			duration,
			AutoPlay(true),
			lerp_fn,
			relative_fn,
			step_fn,
			distance_fn,
		).maybe_bound(self)
	}
}

impl<T, TVal> DoVarProperty<TVal, BaseMarker> for T
	where
		T: WithBaseField + Inherits<Object>,
		TVal: Sized + Clone + ToGodot + FromGodot + SpireLerp,
{
	fn do_var_property(
		&self,
		property: impl AsArg<NodePath>,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Property<Variant>> {
		let lerp_fn = |from: &Variant, to: &Variant, t: f64| -> Variant {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();

			TVal::spire_lerp(&from, &to, t).to_variant()
		};

		let relative_fn = |value_at_obj: &Variant, previous_calc: &Variant, next_calc: &Variant| -> Variant {
			let value_at_obj = value_at_obj.to::<TVal>();
			let previous_calc = previous_calc.to::<TVal>();
			let next_calc = next_calc.to::<TVal>();

			TVal::add_relative(&value_at_obj, &previous_calc, &next_calc).to_variant()
		};

		let step_fn = |from: &Variant, to: &Variant, speed: f64, t: f64| -> (Variant, StepResult) {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();

			let (step, result) = TVal::spire_step(&from, &to, speed, t);
			(step.to_variant(), result)
		};

		let distance_fn = |from: &Variant, to: &Variant| -> f64 {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();

			TVal::spire_distance(&from, &to)
		};

		SpireTween::<Property<Variant>>::new::<TVal>(
			property,
			&self.to_gd(),
			end_val,
			duration,
			AutoPlay(true),
			lerp_fn,
			relative_fn,
			step_fn,
			distance_fn,
		).maybe_bound(&self.to_gd())
	}
}