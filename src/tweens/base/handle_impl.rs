use crate::internal::*;
use crate::tweens::handle::FetchError;

#[allow(private_bounds)]
impl<T: ValidTween> SpireHandle<T> {
	pub fn bound_node(&mut self) -> Result<Option<Gd<Node>>, FetchError> {
		self.map(|tween| tween.bound_node.clone())
	}
	
	pub fn set_bound_node(
		&mut self,
		node: Gd<impl Inherits<Node>>,
	) -> Result<(), FetchError> {
		self.map(|tween| tween.bound_node = Some(node.clone().upcast()))
	}
	
	pub fn clear_bound_node(&mut self) -> Result<(), FetchError> {
		self.map(|tween| tween.bound_node = None)
	}
	
	pub fn delay(&mut self) -> Result<f64, FetchError> {
		self.map(|tween| tween.delay)
	}
	
	pub fn set_delay(&mut self, delay: f64) -> Result<(), FetchError> {
		self.map(|tween| tween.delay = delay)
	}
	
	pub fn speed_scale(&mut self) -> Result<f64, FetchError> {
		self.map(|tween| tween.speed_scale)
	}
	
	pub fn set_speed_scale(&mut self, speed_scale: f64) -> Result<(), FetchError> {
		self.map(|tween| tween.speed_scale = speed_scale)
	}
	
	pub fn elapsed_time(&mut self) -> Result<f64, FetchError> {
		self.map(|tween| tween.elapsed_time)
	}
	
	pub fn set_elapsed_time(&mut self, elapsed_time: f64) -> Result<(), FetchError> {
		self.map(|tween| tween.elapsed_time = elapsed_time)
	}
	
	pub fn cycle_count(&mut self) -> Result<u32, FetchError> {
		self.map(|tween| tween.cycle_count)
	}
	
	pub fn set_cycle_count(&mut self, cycle_count: u32) -> Result<(), FetchError> {
		self.map(|tween| tween.cycle_count = cycle_count)
	}
	
	pub fn pause_mode(&mut self) -> Result<TweenPauseMode, FetchError> {
		self.map(|tween| tween.pause_mode)
	}
	
	pub fn set_pause_mode(&mut self, pause_mode: TweenPauseMode) -> Result<(), FetchError> {
		self.map(|tween| tween.pause_mode = pause_mode)
	}
	
	pub fn process_mode(&mut self) -> Result<TweenProcessMode, FetchError> {
		self.map(|tween| tween.process_mode)
	}
	
	pub fn set_process_mode(&mut self, process_mode: TweenProcessMode) -> Result<(), FetchError> {
		self.map(|tween| tween.process_mode = process_mode)
	}
	
	pub fn loop_mode(&mut self) -> Result<LoopMode, FetchError> {
		self.map(|tween| tween.loop_mode)
	}
	
	pub fn set_loop_mode(&mut self, loop_mode: LoopMode) -> Result<(), FetchError> {
		self.map(|tween| tween.loop_mode = loop_mode)
	}
	
	pub fn on_finish(&mut self, f: impl FnMut() + 'static) -> Result<(), FetchError> {
		self.map(|tween| tween.calls_on_finish.push(f.into()))
	}
	
	pub fn on_finish_callable(&mut self, callable: Callable) -> Result<(), FetchError> {
		self.map(|tween| tween.calls_on_finish.push(callable.into()))
	}
}