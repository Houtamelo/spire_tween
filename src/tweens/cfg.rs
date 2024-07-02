#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AutoPlay(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TweenState {
	Playing,
	Paused,
	Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
	Finite(u32),
	Infinite,
}

#[derive(Debug, Clone, Copy)]
pub enum Duration {
	Finite(f64),
	Infinite
}
