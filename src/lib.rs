#![feature(inline_const_pat)]
#![feature(trait_alias)]
#![feature(hash_extract_if)]
#![feature(let_chains)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_try_fold)]
#![doc = include_str!("../README.md")]

mod singleton;
mod ease;
mod tweens;
mod extensions;

pub mod prelude {
	#[doc(hidden)]
	pub struct BaseMarker;
	
	pub use crate::ease::Ease;
	pub use crate::extensions::{
		bound_tweens::{CompleteBoundTweens, KillBoundTweens},
		do_delayed_call::{DoDelayedCall, DoDelayedCallable},
		do_method::{DoMethod, DoVarMethod},
		property::{
			DoProperty,
			DoVarProperty,
			types::*,
		},
	};
	pub use crate::tweens::{
		base::SpireTween,
		cfg::{AutoPlay, Duration, LoopMode, TweenState},
		delayed_call::DelayedCall,
		handle::{FetchError, SpireHandle},
		lerping::{LerpMode, SpireLerp},
		method::Method,
		property::Property,
		sequence::Sequence,
		traits::SpireTweener,
	};

	pub type SpireSequence = SpireTween<Sequence>;
	pub type SpireDelayedCall = SpireTween<DelayedCall>;
	pub type SpireProperty<T> = SpireTween<Property<T>>;
	pub type SpireMethod<T> = SpireTween<Method<T>>;
}

#[allow(unused_imports)]
pub(crate) mod internal {
	pub(crate) use crate::prelude::BaseMarker;
	pub(crate) use godot::obj::WithBaseField;
	pub(crate) use godot::meta::AsArg;
	
	pub(crate) use std::any::type_name;
	pub(crate) use std::collections::HashMap;
	pub(crate) use std::fmt::Debug;

	pub(crate) use anyhow::{anyhow, bail};
	pub(crate) use godot::classes::tween::{TweenPauseMode, TweenProcessMode};
	pub(crate) use godot::prelude::*;
	pub(crate) use uuid::Uuid;

	pub(crate) use crate::ease::*;
	pub(crate) use crate::extensions::do_delayed_call::*;
	pub(crate) use crate::extensions::do_method::*;
	pub(crate) use crate::extensions::property::DoProperty;
	pub(crate) use crate::singleton::*;
	pub(crate) use crate::tweens::*;
	pub(crate) use crate::tweens::base::*;
	pub(crate) use crate::tweens::cfg::*;
	pub(crate) use crate::tweens::delayed_call::*;
	pub(crate) use crate::tweens::handle::*;
	pub(crate) use crate::tweens::lerping::*;
	pub(crate) use crate::tweens::method::*;
	pub(crate) use crate::tweens::property::*;
	pub(crate) use crate::tweens::sequence::*;
	pub(crate) use crate::tweens::traits::*;
}
