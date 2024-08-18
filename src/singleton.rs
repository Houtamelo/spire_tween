use std::sync::LazyLock;

use godot::classes::node::ProcessMode;

use crate::internal::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct TweensController {
	base: Base<Node>,
	pub(crate) tweens: HashMap<Uuid, AnyTween>,
}

#[godot_api]
impl INode for TweensController {
	fn ready(&mut self) {
		let mut base = self.base_mut();
		base.set_process_mode(ProcessMode::ALWAYS);
		base.set_process_priority(256);
		base.set_physics_process_priority(256);
	}

	fn process(&mut self, delta_time: f64) {
		self.tick_process(delta_time);
	}

	fn physics_process(&mut self, delta_time: f64) {
		self.tick_physics(delta_time);
	}
}

struct UnsafeWrapper {
	inner: Gd<TweensController>,
}

unsafe impl Send for UnsafeWrapper {}

unsafe impl Sync for UnsafeWrapper {}

static SINGLETON: LazyLock<UnsafeWrapper> =
	LazyLock::new(|| {
		let scene_tree: Gd<SceneTree> = godot::classes::Engine::singleton()
			.get_main_loop()
			.expect("Main loop does not exist")
			.try_cast()
			.expect("Main loop does not inherit `SceneTree`");

		let inner = scene_tree
			.get_root()
			.expect("Root node does not exist")
			.get_node_or_null("tweens_controller".into())
			.expect("Root node does not have child named `tweens_controller`")
			.try_cast()
			.expect("Child of Root `tweens_controller` does not inherit `TweensController`");

		UnsafeWrapper { inner }
	});

impl TweensController {
	fn tick_process(&mut self, delta_time: f64) {
		let is_tree_paused = self.base().get_tree().is_none_or(|tree| tree.is_paused());

		self.tweens.retain(|_, tween| {
			if is_tree_paused {
				tween.tick_independent(delta_time);
			} else {
				tween.tick_process(delta_time);
			}

			match tween.state() {
				| TweenState::Playing | TweenState::Paused => true,
				TweenState::Stopped => false,
			}
		});
	}

	fn tick_physics(&mut self, delta_time: f64) {
		self.tweens.retain(|_, tween| {
			tween.tick_physics(delta_time);

			match tween.state() {
				| TweenState::Playing | TweenState::Paused => true,
				TweenState::Stopped => false,
			}
		});
	}

	pub(crate) fn map_mut<TMap>(
		f: impl FnOnce(&mut TweensController) -> TMap,
	) -> TMap {
		let mut gd = SINGLETON.inner.clone();
		let mut brain = gd.bind_mut();
		f(&mut brain)
	}

	pub(crate) fn register<T: ValidTween>(
		&mut self,
		tween: SpireTween<T>,
	) -> SpireHandle<T> {
		let uuid = Uuid::new_v4();
		if self.tweens.insert(uuid, tween.into()).is_some() {
			godot_warn!("Found tween with already existing uuid: {uuid}");
		}

		SpireHandle::new(uuid)
	}
}
 