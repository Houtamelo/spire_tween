use crate::internal::*;

#[allow(private_bounds)]
impl<T> SpireHandle<Property<T>>
	where
		Property<T>: ValidTween,
		T: TweenableValue,
{
	pub fn property_name(&mut self) -> Result<NodePath, FetchError> {
		self.map(|tween| tween.t.property.clone())
	}

	pub fn set_property_name(&mut self, property_name: NodePath) -> Result<(), FetchError> {
		self.map(|tween| tween.t.property = property_name)
	}

	pub fn target(&mut self) -> Result<Gd<Object>, FetchError> {
		self.map(|tween| tween.t.target.clone())
	}

	pub fn set_target(&mut self, target: Gd<Object>) -> Result<(), FetchError> {
		self.map(|tween| tween.t.target = target)
	}

	pub fn lerp_mode(&mut self) -> Result<LerpMode<T>, FetchError> {
		self.map(|tween| tween.t.lerp_mode.clone())
	}

	pub fn set_lerp_mode(&mut self, lerp_mode: LerpMode<T>) -> Result<(), FetchError> {
		self.map(|tween| tween.t.lerp_mode = lerp_mode)
	}

	pub fn ease(&mut self) -> Result<Ease, FetchError> {
		self.map(|tween| tween.t.ease.clone())
	}

	pub fn set_ease(&mut self, ease: Ease) -> Result<(), FetchError> {
		self.map(|tween| tween.t.ease = ease)
	}

	pub fn end_value(&mut self) -> Result<T, FetchError> {
		self.map(|tween| tween.t.end.clone())
	}

	pub fn set_end_value(&mut self, end_value: T) -> Result<(), FetchError> {
		self.map(|tween| tween.t.end = end_value)
	}

	pub fn is_absolute(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_absolute())
	}

	pub fn is_relative(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_relative())
	}

	pub fn is_speed_based(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_speed_based())
	}
}