# SpireTween - An alternative tweening library for Godot 4.2+

# Warning!
- This package is largely undocumented, this readme is all that there is, and it will likely remain so since the author is the only known user.
  PRs adding documentation will be accepted though.
- This package requires Rust nightly, and it uses the following unstable features:
```rs
#![feature(inline_const_pat)]
#![feature(trait_alias)]
#![feature(hash_extract_if)]
#![feature(let_chains)]
#![feature(is_none_or)]
```

# Summary
- [Why](#why)
- [How it works](#how-it-works)
- [Setup](#setup)
- [Help](#help)


# Why
Spire Tween vs Godot Tween

### Main reason: Ergonomics

#### Comparison 1: Basic Tweening
Goal:
1. Tween a `CanvasItem`'s color with Easing
2. Print message

Godot Tween:
```rs
fn basic_godot(mut node: Gd<Node2D>) {
	let mut godot_tween =
		node.create_tween()
		    .unwrap();	// unwrap 1

	let property_tweener =
		godot_tween
			.tween_property(
				node.clone().upcast(),	// boilerplate
				"modulate".into(),	// Requires manually typing property name
				Color::from_rgb(1.0, 0.0, 0.0).to_variant(),	// No type safety
				2.0
			)
			.unwrap()	// unwrap 2
			.set_ease(EaseType::OUT)
			.unwrap();	// unwrap 3

	let err =	// Will error if you make any typos
		godot_tween.connect(
			"finished".into(),	// boilerplate
			Callable::from_fn(
				"print",	// boilerplate
				|_| {
					godot_print!("Tween finished");
					Ok(Variant::nil())	// boilerplate
				},
			));
}
```

Spire Tween:
```rs
fn basic_spire(node: &Gd<Node2D>) {
	let spire_handle =		// No errors, internally calls `bind_mut` in Singleton, will panic if you violate Rust's mutability rules, more info at the end of this README
		node.do_color(Color::from_rgb(1.0, 0.0, 0.0), 2.0)	// Type safe, no variants 
		    .with_ease(Ease::Out)
		    .on_finish(|| {	// Rust compiler doesn't allow making typos
			    godot_print!("Tween finished");
		    }).register();
}
```

#### Comparison 2: Executing a sequence of tweens
Goal: (Make a CanvasItem "blink")
1. Tween a `CanvasItem`'s color from Black => White, with Easing
2. Tween a `CanvasItem`'s color from White => Black, with Easing

Godot Tween:
```rs
fn sequence_godot(mut node: Gd<Node2D>) {
	let mut godot_tween = 
		node.create_tween()
		    .unwrap();	// unwrap 1
	
	// 1st tween
	godot_tween
		.tween_property(
			node.clone().upcast(),	// boilerplate
			"modulate".into(),	// No type safety, typos can happen
			Color::WHITE.to_variant(),	// No type safety
			2.0
		)
		.unwrap()	// unwrap 2 
		.set_ease(EaseType::IN_OUT)
		.unwrap()	// unwrap 3
		.from(Color::BLACK.to_variant());	// No type safety
	
	// 2nd tween
	godot_tween
		.tween_property(
			node.clone().upcast(),	// boilerplate
			"modulate".into(),	// No type safety, typos can happen
			Color::BLACK.to_variant(),	// No type safety
			2.0,
		)
		.unwrap()	// No type safety
		.set_ease(EaseType::OUT_IN);
}
```

Spire Tween:
```rs
fn sequence_spire(node: &Gd<Node2D>) {
	let mut sequence = SpireSequence::new();
	sequence.append(
		node.do_color(Color::WHITE, 2.0)	// Type safe
		    .starting_at(Color::BLACK)	// Type safe
	);

	sequence.append(node.do_color(Color::BLACK, 2.0));	// Type safe
	
	let handle = sequence.register();	// No unwrap
}
```

#### Comparison 3: Executing tweens in parallel
Goal:
1. Tween `CanvasItem`'s scale and color at the same time, with Easing
2. Tween `CanvasItem`'s scale back to normal, with Easing

Godot Tween
```rs
fn parallel_godot(mut node: Gd<Node2D>) {
	let mut godot_tween = 
		node.create_tween()
		    .unwrap();	// unwrap 1

	godot_tween.set_parallel();
	
	godot_tween
		.tween_property(
			node.clone().upcast(),	// boilerplate
			"modulate".into(),	// No type safety, typos can happen
			Color::RED.to_variant(),	// No type safety
			2.0,
		)
		.unwrap()	// unwrap 2
		.set_ease(EaseType::OUT);
	
	godot_tween
		.tween_property(
			node.clone().upcast(),	// boilerplate
			"scale".into(),	// No type safety, typos can happen
			Vector2::new(5.0, 5.0).to_variant(),	// No type safety
			2.0,
		)
		.unwrap()	// unwrap 3
		.set_ease(EaseType::OUT);
	
	godot_tween.chain();
	
	godot_tween
		.tween_property(
			node.clone().upcast(),	// boilerplate
			"scale".into(),	// No type safety, typos can happen
			Vector2::new(1.0, 1.0).to_variant(),	// No type safety
			2.0,
		)
		.unwrap()	// unwrap 4
		.set_ease(EaseType::IN);
}
```

Spire Tween:
```rs
fn parallel_spire(node: &Gd<Node2D>) {
	let mut sequence = SpireSequence::new();
	sequence.append(
		node.do_color(Color::RED, 2.0)	// Type safe, typos don't compile
			.with_ease(Ease::Out)
	);
	
	sequence.join(
		node.do_scale(Vector2::new(5.0, 5.0), 2.0)	// Type safe, typos don't compile
			.with_ease(Ease::Out)
	);
	
	sequence.append(
		node.do_scale(Vector2::new(1.0, 1.0), 2.0)	// Type safe, typos don't compile
			.with_ease(Ease::In)
	);
	
	let handle = sequence.register();
}
```

#### Comparison 4: Configuring tweens
Godot Tween:
```rs
fn using_godot_tween(mut node: Gd<Node2D>) {
	let mut godot_tween =
		node.create_tween()
		    .unwrap()	// unwrap 1
		    .set_loops_ex()	// Many settings require boilerplate builder patterns
		    .loops(5)
		    .done()
		    .unwrap()	// unwrap 2 
		    .set_process_mode(TweenProcessMode::IDLE)
		    .unwrap()	// unwrap 3
		    .set_speed_scale(2.0)
		    .unwrap()	// unwrap 4
		    .set_pause_mode(TweenPauseMode::BOUND)
		    .unwrap();	// unwrap 5
	
	// Tweening a property
	let mut property_tweener =
		godot_tween
			.tween_property(
				node.clone().upcast(),	// boilerplate
				"modulate".into(),	// Requires manually typing property name
				Color::from_rgb(1.0, 0.0, 0.0).to_variant(),	// No type safety
				2.0,
			).unwrap();	// unwrap 6
	
	property_tweener
		.as_relative()
		.unwrap()	// unwrap 7
		.set_ease(EaseType::IN)// limited to Godot's Ease modes
		.unwrap()	// unwrap 8
		.set_delay(5.0)
		.unwrap();	// unwrap 9
	
	godot_tween
		.connect_ex(
			"finished".into(),	// boilerplate
			Callable::from_object_method(&node, "on_finished"),
		)
		.flags(object::ConnectFlags::DEFERRED.ord() as u32)
		.done();
	
	// Creating a sequence
	godot_tween
		.tween_method(
			Callable::from_object_method(&node, "tweenable_method"),
			2.to_variant(),	// No type safety
			10.to_variant(),	// No type safety
			5.0
		).unwrap();	// unwrap 10
}
```

Spire Tween:
```rs
fn configuring_spire(node: &Gd<Node2D>) {
	let spire_tween =
		node.do_color(Color::from_rgb(1.0, 0.0, 0.0), 2.0)	// Type safe, no variants
			// Just like in Godot, you don't have to manually set these if you want to use defaults
			// this is just to show that you can
			.looped(5)
		    .with_process_mode(TweenProcessMode::IDLE)
		    .with_pause_mode(TweenPauseMode::BOUND)
		    .with_delay(5.0)
		    .with_speed_scale(2.0)
		    .with_ease(Ease::In)	// use `keyframe` crate for easing, providing commonly-used easing as well as allowing custom ones
		    .as_relative(Color::BLACK)	// relative allows setting a custom origin
		    .as_speed_based(20.0)
			// do_color automatically binds its lifetime to `node`, the object you called this on, but you can also 
			// bind it to other godot objects instead of just `node`
			.bound_to(other_object)
		    .on_finish(|| { godot_print!("Finished"); })	// direct closures
		    .on_finish(Callable::from_object_method(&node, "on_finish"));	// callables work too

	let mut handle
		: SpireHandle<Property<Color>>	// Handle knows exact tween type, providing type safety 
		= spire_tween.register();	// Same as `done()`, only needs to be called once, no unwrapping

	// You can use the handle to read/mutate the tween on the fly
	// Handles don't guarantee that what they point at exists
	let fetch_result: Result<(), FetchError> =
		handle.on_finish(|| {
			godot_print!("Managed to connect on_finish on the fly!")
		});

	match fetch_result {
		Ok(()) => {}
		Err(fetch_err) => {
			match fetch_err {	// Error explaining exactly what went wrong
				FetchError::NotFound => {}
				FetchError::TypeMismatch { expected, found } => {
					godot_error!("Could not fetch tween expected type {expected:?}, found {found:?}");
				}
			}
		}
	}

	// Handles allow you to perform almost all the actions you can perform as if you're building the tween
	// Everything returns a result though
	let _ = handle.stop();

	// If you want to access the tween directly, you can call map
	let fetch_result: Result<TweenState, FetchError> =
		handle.map(|tween: &mut SpireTween<Property<Color>>| {
			tween.t.ease = Ease::In;
			tween.delay = 2.0;
			// ..

			// Need to read something on the tween? Return it on map:
			tween.state()
		});
}
```

# How it works

`register()` attempts to access the [TweensController](src/singleton.rs) autoload, inserting the tween on its internal list.
It also returns a `SpireHandle<T>`, which you can use to access the tween later, allowing you to read its status, change settings on the fly, etc.

The operation will **panic** if cTweensControllerHow it works is inaccessible, which can happen if it doesn't exist, or if it is already borrowed somewhere else.

However, this should not happen provided you follow some rules mentioned at the end of this README. 

`SpireTween` provides several methods for configuring the behavior of your tweens.
Although the library is undocumented, the names are fairly intuitive, following the pattern: "do_" + godot_property_name.

You should make all the configurations before registering the Tween, modifying the tween after registering can lead to unpredictable behavior.
I do not recommend attempting to register/access tweens outside Godot's main thread.

More Examples:
```rs
let node: Gd<Node2D> = ..;
let mut tween = owner.do_move_x(2000.0, 5.0);

let origin = 0.0; // since relative tweens don't have a starting point, the origin parameter is necessary for the tween to compare how much it needs to move between each frame
tween = tween.as_relative(origin);

// If you want to set a specific starting point: (if you don't, the starting point will be evaluated when the tween starts.
tween = tween.starting_at(500.0);

tween = tween.with_delay(10.0);

let speed = 100.0; // Speed is always in units per second, 100. speed means position:x increases by 100 each second.
tween = tween.as_speed_based(speed);
```

You can chain all these methods together:
```rs
let handle =
	node.do_global_position(Vector2::new(1920., 1080.), 5.0)
		.with_delay(10.0)
		.starting_at(Vector2::new(0., 0.))
		.looped(10)
		.on_finish(|| { godot_print!("Tween Finished"); })
		.register();
```

Property tweeners are merely wrappers around the trait [DoProperty](https://github.com/Houtamelo/spire_tween/blob/main/src/extensions/property/mod.
rs).
`do_property` automatically binds the node to the tween, this means that the tween will automatically "die" when the node is deleted.

DoProperty uses `set_indexed()`, so property paths are also valid:
```rs
// Tween only the Z position
let tween = node.do_property("position:z", 20., 10.);
```

If you want to use values that aren't natively supported by the library, you can use `DoVarProperty/DoVarMethod`.
```rs
let my_custom_property = ..;
let my_custom_end_val = ..;
let duration = 5.0;

// the only difference is that the method is called do_property_var()
let tween = node.do_var_property(my_custom_property, my_custom_end_val, duration);
```

You can also tween methods:
```rs
// Imagine you have a method like this...
#[func]
fn _set_fill(&mut self, value: f64) {
	self.base_mut().set_value(value);
}

// You can tween it like this
let start = 0.;
let end = 1.;
let duration = 8.;
let tween = node.do_method("_set_fill", start, end, duration);
```

If you just need to call a method once, with a delay, you can:
```rs
#[func]
fn _start_game(&mut self, player_name: String, difficulty: Difficulty) {
	...
}

let player_name = "Houtamelo";
let difficulty = Difficulty::GameDeveloper;
let delay = 24.;
let args = Array::from(&[player_name.to_variant(), difficulty.to_variant()]);

let callable = Callable::from_object_method(node, "_start_game").bindv(args);

let tween = node.do_delayed_call(callable, delay);

// Closures are also accepted:
let tween = node.do_delayed_call(|| node._start_game(player_name, difficulty), delay);
```

## Never forget to register your tweens! They will just be dropped otherwise:
```rs
let handle = tween.register();
```

We also have sequences, that work very much like DoTween's:
```rs
let mut sequence = SpireSequence::new();
sequence.append(owner.do_move_x(640.0, 5.0));
sequence.join(owner.do_fade(1.0, 4.0));
sequence.append(owner.do_global_move(Vector2::new(124., 256.));
sequence.insert(8.0, owner.do_scale(Vector2::new(10., 10.));

// SpireSequence is just an alias for SpireTween<Sequence>, which means you can also nest them 
let nested_sequence = {
	let mut nested = SpireSequence::new();
	nested.append(owner.do_color(Color::BLUE, 2.0));
	nested.append(owner.do_move_y(Vector2::new(0, 69));
	nested
};

sequence.append(nested_sequence);

let handle = sequence.register();
```

Need to kill a tween?
```rs
let handle: SpireHandle<DelayedCall> = ..;
handle.kill();

// As you may have guessed from the types, sequences are also tweens:
let sequence_handle: SpireHandle<Sequence> = ..;
sequence_handle.kill();
```

Need to complete a tween immediately?
```rs
handle.complete();
```

# Know that...
- `spire_tween` does not interact in any way with Godot's built-in tweens, it runs on a standalone autoload that processes the registered tweens on 
  its `_process(f64)` and `_physics_process(f64)` virtual methods.

# Setup
The setup process assumes you're already familiar with how [GDExtension](https://godot-rust.github.io/) works.

### Step 1
Add `spire_tween` to your project's dependencies:
run `cargo add spire_tween`
or
add following line to your Cargo.toml:
[dependencies]
`spire_tween = "0.1"`

You'll most likely going to want to use the latest version, as it will contain the latest bug-fixes.

### Step 2
Once you build your dynamic library, the class `TweensController` should be automatically registered with Godot the next time you open the editor.
Create an empty scene with the root named "tweens_controller" and type `TweensController`, then add that as your autoload.  

### Warning!: The node must be named "tweens_controller", otherwise Rust will panic whenever attempting to register a tween, crashing your game.

# Rules
Do not attempt to borrow `TweensController` in two places at the same time, this will happen if:
- You register a tween from: inside a callback provided on `on_finish` || calls triggered in one of your `do_property` or `do_method` calls
- You use a handle to access a tween from: inside a callback provided on `on_finish` || calls triggered in one of your `do_property` or `do_method`
  calls
- You're registering/accessing tweens from different threads

# Help
Open an issue or send me a message on discord: `houtamelo`
