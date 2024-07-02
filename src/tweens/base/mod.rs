pub mod builder;
pub mod methods;
pub mod handle_impl;

use crate::internal::*;

#[must_use]
pub struct SpireTween<T> {
	pub bound_node: Option<Gd<Node>>,
	pub delay: f64,
	pub speed_scale: f64,
	pub elapsed_time: f64,
	pub cycle_count: u32,
	pub pause_mode: TweenPauseMode,
	pub process_mode: TweenProcessMode,
	pub loop_mode: LoopMode,
	pub calls_on_finish: Vec<DelayedCall>,
	pub t: T,
	pub(crate) state: TweenState,
}
