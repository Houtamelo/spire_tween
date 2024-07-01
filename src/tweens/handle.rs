use std::marker::PhantomData;

use crate::internal::*;

#[derive(Debug, Copy, Clone)]
pub struct SpireHandle<T> {
	pub(crate) id: Uuid,
	_marker: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub enum FetchError {
	NotFound,
	TypeMismatch { expected: &'static str, found: &'static str },
}

#[allow(private_bounds)]
impl<T> SpireHandle<T>
	where
		T: ValidTween,
{
	pub fn new(id: Uuid) -> Self { Self { id, _marker: PhantomData } }

	pub fn map<TMap>(
		&mut self,
		f: impl FnOnce(&mut SpireTween<T>) -> TMap,
	) -> Result<TMap, FetchError> {
		TweensController::map_mut(|brain| {
			let any_tween =
				brain.tweens
				     .get_mut(&self.id)
				     .ok_or(FetchError::NotFound)?;

			let found_type = any_tween.inner_type_name();

			let tween =
				T::ref_from_any(any_tween)
					.ok_or_else(|| {
						FetchError::TypeMismatch {
							expected: type_name::<T>(),
							found: found_type,
						}
					})?;

			Ok(f(tween))
		})
	}

	pub fn is_valid(&mut self) -> bool {
		self.map(|_| {}).is_ok()
	}

	pub fn kill(&self) {
		TweensController::map_mut(|brain| {
			brain.tweens.remove(&self.id);
		});
	}

	pub fn complete(&self) {
		TweensController::map_mut(|brain| {
			if let Some(tween) = brain.tweens.remove(&self.id) {
				tween.complete();
			}
		});
	}

	pub fn claim(self) -> Result<SpireTween<T>, FetchError> {
		TweensController::map_mut(|brain| {
			let any_tween =
				brain.tweens
				     .remove(&self.id)
				     .ok_or(FetchError::NotFound)?;

			let found_type = any_tween.inner_type_name();

			let tween =
				T::from_any(any_tween)
					.map_err(|mismatched_tween| {
						brain.tweens.insert(self.id, mismatched_tween);

						FetchError::TypeMismatch {
							expected: type_name::<T>(),
							found: found_type,
						}
					})?;

			Ok(tween)
		})
	}
}

#[allow(private_bounds)]
impl<T> SpireHandle<T>
	where
		T: ValidTween,
		SpireTween<T>: Tick,
{
	pub fn state(&mut self) -> Result<State, FetchError> {
		self.map(|tween| tween.state())
	}

	pub fn set_state(&mut self, state: State) -> Result<(), FetchError> {
		self.map(|tween| tween.set_state(state))
	}

	pub fn is_playing(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_playing())
	}

	pub fn play(&mut self) -> Result<(), FetchError> {
		self.map(|tween| { tween.play(); })
	}

	pub fn is_paused(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_paused())
	}

	pub fn pause(&mut self) -> Result<(), FetchError> {
		self.map(|tween| { tween.pause(); })
	}

	pub fn is_stopped(&mut self) -> Result<bool, FetchError> {
		self.map(|tween| tween.is_stopped())
	}

	pub fn stop(&mut self) -> Result<(), FetchError> {
		self.map(|tween| { tween.stop(); })
	}

	/*
	pub fn do_absolute_step(&self, delta: f64) -> Result<()> {
		self.map_mut(|tween| { tween.do_absolute_step(delta) })
	}

	pub fn do_scaled_step(&self, delta: f64) -> Result<()> {
		self.map_mut(|tween| { tween.do_scaled_step(delta) })
	}

	pub fn seek(&self, time: f64) -> Result<()> {
		self.map_mut(|tween| { tween.seek(time) })
	}
	*/
}