# SpireTween - An alternative tweening library for Godot 4.2+

# Warning!
- This package is not thoroughly tested, it may contain game-breaking bugs. That being said, I use it everyday on all my projects, and I am 
  committed to fixing any reported bugs as fast as I can, at least until 2026.
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
- [Usage](#usage)
- [Setup](#setup)
- [Help](#help)


# Usage
Here's how you can tween a node's color:

```rs
use spire_tween::prelude::*;
use godot::prelude::*;

// Assuming you have a reference to something that inherits `CanvasItem`
let node: Gd<Sprite> = ..;

// Must-have parameters
let color = Color::from_rgb(1.0, 0.0, 0.0);
let duration_seconds = 2.0;

// This creates the tween's configuration 
let tween = node.do_color(color, duration); // Type of `tween` == SpireTween<Property<Color>> 

// Making the tween run requires registering it in the `TweensController` autoload
let handle = tween.register(); // Type of `handle` == SpireHandle<Property<Color>>
```

`register()` attempts to access the [TweensController](src/singleton.rs) autoload, inserting the tween on its internal list.
It also returns a `SpireHandle`, which you can use to access the tween later, allowing you to read its status, change settings on the fly, etc.

The operation will **panic** if TweensController is inaccessible, which can happen if it doesn't exist, or if it is already borrowed somewhere else.

However, this should not happen provided you follow some rules mentioned at the end of this README. 

`SpireTween` provides several methods for configuring the behavior of your tweens.
Although the library is undocumented, the names are fairly intuitive, following the pattern: "do_" + godot_property_name.

You should make all the configurations before registering the Tween, modifying the tween after registering can lead to unpredictable behavior.
I do not recommend attempting to register/access tweens outside Godot's main thread.

# Examples
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
let mut sequence = SpireTween::<Sequence>::new();
sequence.append(owner.do_move_x(640.0, 5.0));
sequence.join(owner.do_fade(1.0, 4.0));
sequence.append(owner.do_global_move(Vector2::new(124., 256.));
sequence.insert(8.0, owner.do_scale(Vector2::new(10., 10.));

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
