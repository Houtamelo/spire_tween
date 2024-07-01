use crate::internal::*;

pub trait DoMethod<TVal> {
	type Return;

	fn do_method(
		&self,
		method: impl Into<StringName>,
		start_val: TVal,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Method<Self::Return>>;
}

impl<T, TVal> DoMethod<TVal> for Gd<T>
	where
		T: GodotClass + Inherits<Object>,
		TVal: TweenableValue + SpireLerp,
		Method<TVal>: ValidTween,
{
	type Return = TVal;

	fn do_method(
		&self,
		method: impl Into<StringName>,
		start_val: TVal,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Method<TVal>> {
		SpireTween::<Method<TVal>>::new(
			method.into(),
			self.clone(),
			start_val,
			end_val,
			duration,
			AutoPlay(true),
		).maybe_bound(self.clone())
	}
}

pub trait DoVarMethod<TVal: ToGodot> {
	fn do_var_method(
		&self,
		method: impl Into<StringName>,
		start_val: TVal,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Method<Variant>>;
}

impl<T, TVal> DoVarMethod<TVal> for Gd<T>
	where
		T: GodotClass + Inherits<Object>,
		TVal: Sized + Clone + ToGodot + FromGodot + SpireLerp,
{
	fn do_var_method(
		&self,
		method: impl Into<StringName>,
		start_val: TVal,
		end_val: TVal,
		duration: f64,
	) -> SpireTween<Method<Variant>> {
		let lerp_fn = |from: &Variant, to: &Variant, t: f64| -> Variant {
			let from = from.to::<TVal>();
			let to = to.to::<TVal>();

			TVal::spire_lerp(&from, &to, t).to_variant()
		};

		SpireTween::<Method<Variant>>::new::<TVal>(
			method.into(),
			self.clone(),
			start_val,
			end_val,
			duration,
			AutoPlay(true),
			lerp_fn,
		).maybe_bound(self.clone())
	}
}