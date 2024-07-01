#![feature(inline_const_pat)]
#![feature(trait_alias)]
#![feature(hash_extract_if)]
#![feature(let_chains)]
#![feature(is_none_or)]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_try_fold)]

mod singleton;
mod ease;
mod tweens;
mod extensions;

pub mod prelude {
	pub type SpireSequence = SpireTween<Sequence>;
	pub type SpireDelayedCall = SpireTween<DelayedCall>;
	pub type SpireProperty<T> = SpireTween<Property<T>>;
	pub type SpireMethod<T> = SpireTween<Method<T>>;
	
	pub use crate::extensions::{
		property::{
			types::*,
			DoProperty,
			DoVarProperty,
		},
		do_method::{DoMethod, DoVarMethod},
		do_delayed_call::DoDelayedCall,
		bound_tweens::{CompleteBoundTweens, KillBoundTweens},
	};
	
	pub use crate::ease::Ease;
	
	pub use crate::tweens::{
		cfg::{AutoPlay, State, LoopMode, Duration},
		handle::{SpireHandle, FetchError},
		lerping::{SpireLerp, LerpMode},
		traits::SpireTweener,
		base::SpireTween,
		delayed_call::DelayedCall,
		method::Method,
		property::Property,
		sequence::Sequence,
	};
}

#[allow(unused_imports)]
pub(crate) mod internal {
	pub(crate) use std::any::type_name;
	pub(crate) use std::collections::HashMap;
	pub(crate) use std::fmt::Debug;
	pub(crate) use anyhow::{anyhow, bail, Result};
	pub(crate) use godot::classes::EditorPlugin;
	pub(crate) use godot::classes::tween::{TweenPauseMode, TweenProcessMode};
	pub(crate) use godot::prelude::*;
	pub(crate) use uuid::Uuid;
	pub(crate) use crate::singleton::*;
	pub(crate) use crate::tweens::*;
	pub(crate) use crate::tweens::base::*;
	pub(crate) use crate::tweens::cfg::*;
	pub(crate) use crate::tweens::delayed_call::*;
	pub(crate) use crate::tweens::lerping::*;
	pub(crate) use crate::tweens::method::*;
	pub(crate) use crate::tweens::property::*;
	pub(crate) use crate::tweens::sequence::*;
	pub(crate) use crate::tweens::traits::*;
	pub(crate) use crate::tweens::handle::*;
	pub(crate) use crate::ease::*;
	pub(crate) use crate::extensions::do_delayed_call::*;
	pub(crate) use crate::extensions::do_method::*;
	pub(crate) use crate::extensions::property::DoProperty;
}
