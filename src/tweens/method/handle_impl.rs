use crate::internal::*;
use crate::tweens::handle::FetchError;

#[allow(private_bounds)]
impl<T> SpireHandle<Method<T>>
	where
		Method<T>: ValidTween,
		T: TweenableValue,
{
	pub fn method_name(&mut self) -> Result<StringName, FetchError> {
		self.map(|tween| tween.t.method.clone())
	}

	pub fn set_method_name(&mut self, method_name: StringName) -> Result<(), FetchError> {
		self.map(|tween| tween.t.method = method_name)
	}

	pub fn target(&mut self) -> Result<Gd<Object>, FetchError> {
		self.map(|tween| tween.t.target.clone())
	}

	pub fn set_target(&mut self, target: Gd<Object>) -> Result<(), FetchError> {
		self.map(|tween| tween.t.target = target)
	}

	pub fn duration(&mut self) -> Result<f64, FetchError> {
		self.map(|tween| tween.t.duration)
	}

	pub fn set_duration(&mut self, duration: f64) -> Result<(), FetchError> {
		self.map(|tween| tween.t.duration = duration)
	}

	pub fn ease(&mut self) -> Result<Ease, FetchError> {
		self.map(|tween| tween.t.ease.clone())
	}

	pub fn set_ease(&mut self, ease: Ease) -> Result<(), FetchError> {
		self.map(|tween| tween.t.ease = ease)
	}

	pub fn start_value(&mut self) -> Result<T, FetchError> {
		self.map(|tween| tween.t.start.clone())
	}

	pub fn set_start_value(&mut self, start: T) -> Result<(), FetchError> {
		self.map(|tween| tween.t.start = start)
	}

	pub fn end_value(&mut self) -> Result<T, FetchError> {
		self.map(|tween| tween.t.end.clone())
	}

	pub fn set_end_value(&mut self, end: T) -> Result<(), FetchError> {
		self.map(|tween| tween.t.end = end)
	}
}