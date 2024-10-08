use crate::internal::*;

pub trait SpireTweener {
	fn state(&self) -> TweenState;
	fn set_state(&mut self, state: TweenState);

	fn play(&mut self) { self.set_state(TweenState::Playing); }
	fn pause(&mut self) { self.set_state(TweenState::Paused); }
	fn stop(&mut self) { self.set_state(TweenState::Stopped); }

	fn is_playing(&self) -> bool { self.state() == TweenState::Playing }
	fn is_paused(&self) -> bool { self.state() == TweenState::Paused }
	fn is_stopped(&self) -> bool { self.state() == TweenState::Stopped }
}

pub(crate) trait TweenerStep: SpireTweener {
	// todo! Convert return to f64 (< 0 if invalid)
	fn advance_time(&mut self, delta_time: f64) -> Option<f64>;
	fn complete(self);
}

pub(crate) trait Tick: TweenerStep {
	fn tick_process(&mut self, delta_time: f64);
	fn tick_physics(&mut self, delta_time: f64);
	fn tick_independent(&mut self, delta_time: f64);
}

pub(crate) trait InnerTypeName {
	fn inner_type_name(&self) -> &'static str;
}

pub(crate) trait TweenConvert: Sized {
	fn ref_from_any(tween: &mut AnyTween) -> Option<&mut SpireTween<Self>>;
	fn from_any(tween: AnyTween) -> Result<SpireTween<Self>, AnyTween>;
}


pub(crate) trait ValidTween = where Self: Sized + TweenConvert, AnyTween: From<SpireTween<Self>>, SpireTween<Self>: Into<AnyTween>;

pub(crate) trait TweenableValue = Sized + Clone + ToGodot + FromGodot;
