use crate::internal::*;

impl SpireHandle<Sequence> {
	#[allow(private_bounds)]
	pub fn append(&mut self, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
		self.map(|seq| seq.append(tween))
	}
	
	pub fn append_call(&mut self, call: impl Into<DelayedCall>) -> Result<(), FetchError> {
		self.map(|seq| seq.append_call(call))
	}
	
	pub fn append_interval(&mut self, duration: f64) -> Result<(), FetchError> {
		self.map(|seq| seq.append_interval(duration))
	}

	#[allow(private_bounds)]
	pub fn join(&mut self, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
		self.map(|seq| seq.join(tween))
	}
	
	pub fn join_call(&mut self, call: impl Into<DelayedCall>) -> Result<(), FetchError> {
		self.map(|seq| seq.join_call(call))
	}

	#[allow(private_bounds)]
	pub fn insert(&mut self, time: f64, tween: impl Into<AnyTween>) -> Result<(), FetchError> {
		self.map(|seq| seq.insert(time, tween))
	}
	
	pub fn insert_call(&mut self, time: f64, call: impl Into<DelayedCall>) -> Result<(), FetchError> {
		self.map(|seq| seq.insert_call(time, call))
	}
}