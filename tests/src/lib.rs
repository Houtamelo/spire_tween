#![feature(inline_const_pat)]
#![allow(clippy::absurd_extreme_comparisons)]

use std::fmt::{Display, Formatter};

use godot::classes::{Button, Control};
use godot::prelude::*;

use spire_tween::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}



struct Report {
	time: f64,
	x: Option<String>,
	y: Option<String>,
	color: Option<String>,
}

impl Display for Report {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{:.2}:", self.time)?;

		if let Some(err) = &self.x {
			writeln!(f, "\tX: {}", err)?;
		}

		if let Some(err) = &self.y {
			writeln!(f, "\tY: {}", err)?;
		}

		if let Some(err) = &self.color {
			writeln!(f, "\tColor: {}", err)?;
		}

		Ok(())
	}
}

impl Report {
	fn generate(sprite: &Gd<Node2D>, time: f64, expected_x: f64, expected_y: f64, expected_r: f64) -> Option<Report> {
		let owner_x = sprite.get_position().x as f64;
		let x =
			if f64::abs(owner_x - expected_x) > (owner_x.abs() + expected_x.abs()) / 1000. {
				Some(format!("Expected x: {}, actual x: {}", expected_x, owner_x))
			} else {
				None
			};

		let owner_y = sprite.get_position().y as f64;
		let y =
			if f64::abs(owner_y - expected_y) > (owner_y.abs() + expected_y.abs()) / 1000. {
				Some(format!("Expected y: {}, actual y: {}", expected_y, owner_y))
			} else {
				None
			};

		let owner_r = sprite.get_modulate().r as f64;
		let color =
			if f64::abs(owner_r - expected_r) > (owner_r.abs() + expected_r.abs()) / 1000. {
				Some(format!("Expected color r: {}, actual color r: {}", expected_r, owner_r))
			} else {
				None
			};

		if x.is_some() || y.is_some() || color.is_some() {
			Some(Report { x, y, color, time })
		} else {
			None
		}
	}
}

#[derive(Debug, Copy, Clone, Default)]
enum State {
	Running { test: Test },
	PrintingReports,
	#[default] Finished,
}

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct Tester {
	base: Base<Node2D>,
	sequence: Option<SpireHandle<Sequence>>,
	reports: Vec<Report>,
	frame_count: i64,
	state: State,
	sprite: Option<Gd<Node2D>>,
}

const D_1: f64 = 4.;
const D_15: f64 = D_1 * 1.5;
const D_2: f64 = D_1 * 2.0;

#[derive(Debug, Copy, Clone, GodotConvert)]
#[godot(via = GString)]
enum Test {
	Absolute,
	SpeedBased,
	Relative,
	Delay,
}

impl Test {
	fn report(&self, tester: &mut Tester, sprite: &Gd<Node2D>, time: f64) {
		match self {
			Test::Absolute => {
				let expected_x =
					match time {
						..=D_1 => {
							f64::spire_lerp(&0., &1000., time / D_1)
						}
						..=D_2 => {
							f64::spire_lerp(&1000., &-500., (time - D_1) / D_1)
						}
						_ => -500.
					};

				let expected_y =
					match time {
						..=D_1 => {
							f64::spire_lerp(&0., &-1000., time / D_1)
						}
						..=D_15 => {
							-1000.
						}
						..=D_2 => {
							f64::spire_lerp(&-1000., &0., (time - D_15) / (D_2 - D_15))
						}
						_ => 0.
					};

				let expected_color_r =
					match time {
						..=D_2 => {
							f64::spire_lerp(&0., &0.5, time / D_2)
						}
						_ => 0.5
					};

				if let Some(report) = Report::generate(sprite, time, expected_x, expected_y, expected_color_r) {
					tester.reports.push(report)
				}
			}
			Test::SpeedBased => {
				let expected_x =
					match time {
						..=10.0 => {
							f64::spire_step(&0., &1000., 100., time).0
						}
						..=19.0 => {
							f64::spire_step(&1000., &-800., 200., time - 10.).0
						}
						_ => -800.
					};

				let expected_y =
					match time {
						..=10.0 => {
							f64::spire_step(&0., &-1000., 100., time).0
						}
						..=17.0 => {
							f64::spire_step(&-1000., &400., 200., time - 10.).0
						}
						_ => 400.
					};

				let expected_color_r =
					match time {
						..=10.0 => {
							f64::spire_step(&0., &0.5, 0.05, time).0
						}
						..=12.0 => {
							0.5
						}
						..=12.5 => {
							f64::spire_step(&0.5, &0., 1., time - 12.).0
						}
						_ => 0.
					};

				if let Some(report) = Report::generate(sprite, time, expected_x, expected_y, expected_color_r) {
					tester.reports.push(report)
				}
			}
			Test::Relative => {
				let expected_x =
					match time {
						..=D_1 => {
							f64::spire_lerp(&0., &1000., time / D_1)
						}
						..=D_2 => {
							1000. - 500. * (time - D_1) / D_1
						}
						_ => 500.
					};

				let expected_y =
					match time {
						..=2.0 => {
							f64::spire_lerp(&0., &-1000., time / D_1)
						}
						..=D_1 => {
							f64::spire_lerp(&0., &-1000., time / D_1)
								+ f64::spire_lerp(&0., &1500., (time - 2.0) / D_1)
						}
						..=const { D_1 + 2.0 } => {
							-1000. + f64::spire_lerp(&0., &1500., (time - 2.0) / D_1)
						}
						_ => 500.
					};

				let expected_color_r =
					match time {
						..=D_2 => {
							f64::spire_lerp(&0., &1., time / D_2)
						}
						_ => 1.
					};

				if let Some(report) = Report::generate(sprite, time, expected_x, expected_y, expected_color_r) {
					tester.reports.push(report)
				}
			}
			Test::Delay => {
				let expected_x =
					match time {
						..=3. => 0.,
						..=const { D_1 + 3. } => {
							f64::spire_lerp(&0., &1000., (time - 3.) / D_1)
						}
						..=const { D_1 + 8. } => 1000.,
						..=const { D_2 + 8. } => {
							f64::spire_lerp(&1000., &-500., (time - D_1 - 8.) / D_1)
						}
						_ => -500.
					};

				let expected_y =
					match time {
						..=3. => 0.,
						..=const { D_1 + 3. } => {
							f64::spire_lerp(&0., &-1000., (time - 3.) / D_1)
						}
						..=const { D_15 + 3. } => {
							-1000.
						}
						..=const { D_2 + 3. } => {
							f64::spire_lerp(&-1000., &0., (time - D_15 - 3.) / (D_2 - D_15))
						}
						_ => 0.
					};

				let expected_color_r =
					match time {
						..=0.5 => 0.,
						..=const { D_2 + 0.5 } => {
							f64::spire_lerp(&0., &0.5, (time - 0.5) / D_2)
						}
						_ => 0.5
					};

				if let Some(report) = Report::generate(sprite, time, expected_x, expected_y, expected_color_r) {
					tester.reports.push(report)
				}
			}
		}
	}

	#[must_use]
	fn start(&self, sprite: &Gd<Node2D>) -> SpireHandle<Sequence> {
		match self {
			Test::Absolute => {
				sprite.do_color_r(0.5, D_2)
				      .register();

				let mut seq = SpireTween::<Sequence>::new().bound_to(sprite);
				seq.append(sprite.do_move_x(1000.0, D_1));
				seq.join(sprite.do_move_y(-1000.0, D_1));
				seq.append(sprite.do_move_x(-500.0, D_1));
				seq.insert(D_15, sprite.do_move_y(0., D_1).with_speed_scale(2.));
				seq.register()
			}
			Test::SpeedBased => {
				sprite.do_color_r(0.5, 0.)
				      .as_speed_based(0.05)
				      .register();

				let mut seq = SpireTween::<Sequence>::new().bound_to(sprite);
				seq.append(sprite.do_move_x(1000.0, D_1).as_speed_based(100.));
				seq.join(sprite.do_move_y(-1000.0, D_1).as_speed_based(100.));
				seq.append(sprite.do_move(Vector2::new(-800., 400.), D_1).as_speed_based(200.));
				seq.insert(12.0, sprite.do_color_r(0., 0.).as_speed_based(1.));
				seq.register()
			}
			Test::Relative => {
				sprite.do_color_r(1., D_2)
				      .register();

				let mut seq = SpireTween::<Sequence>::new().bound_to(sprite);
				seq.append(sprite.do_move_x(1000.0, D_1));
				seq.join(sprite.do_move_y(-1000.0, D_1).as_relative(0.));
				seq.append(sprite.do_move_x(-500.0, D_1).as_relative(0.));
				seq.insert(2.0, sprite.do_move_y(1500., D_1).as_relative(0.));
				seq.register()
			}
			Test::Delay => {
				sprite.do_color_r(0.5, D_2)
				      .with_delay(0.5)
				      .register();

				let mut seq = SpireTween::<Sequence>::new().bound_to(sprite).with_delay(3.0);
				seq.append(sprite.do_move_x(1000.0, D_1));
				seq.join(sprite.do_move_y(-1000.0, D_1));
				seq.append(sprite.do_move_x(-500.0, D_1).with_delay(5.0));
				seq.insert(D_15, sprite.do_move_y(0., D_1).with_speed_scale(2.));
				seq.register()
			}
		}
	}
}

#[godot_api]
impl INode2D for Tester {
	fn init(base: Base<Self::Base>) -> Self {
		Self {
			base,
			sequence: None,
			reports: Vec::new(),
			frame_count: 1,
			state: State::Finished,
			sprite: None,
		}
	}

	fn ready(&mut self) {
		let base = self.base().to_godot();

		self.sprite = Some(base.get_node_as("sprite"));

		let buttons =
			base.get_node_as::<Control>("test_buttons");

		buttons.get_node_as::<Button>("std")
		       .connect_ex("pressed".into(), Callable::from_object_method(&base, "_start_test").bindv(Array::from(&[Test::Absolute.to_variant()])))
		       .flags(godot::classes::object::ConnectFlags::DEFERRED.ord() as u32)
		       .done();

		buttons.get_node_as::<Button>("speed_based")
		       .connect_ex("pressed".into(), Callable::from_object_method(&base, "_start_test").bindv(Array::from(&[Test::SpeedBased.to_variant()])))
		       .flags(godot::classes::object::ConnectFlags::DEFERRED.ord() as u32)
		       .done();

		buttons.get_node_as::<Button>("relative")
		       .connect_ex("pressed".into(), Callable::from_object_method(&base, "_start_test").bindv(Array::from(&[Test::Relative.to_variant()])))
		       .flags(godot::classes::object::ConnectFlags::DEFERRED.ord() as u32)
		       .done();

		buttons.get_node_as::<Button>("delay")
		       .connect_ex("pressed".into(), Callable::from_object_method(&base, "_start_test").bindv(Array::from(&[Test::Delay.to_variant()])))
		       .flags(godot::classes::object::ConnectFlags::DEFERRED.ord() as u32)
		       .done();
	}

	fn process(&mut self, _delta: f64) {
		match &mut self.state {
			State::Running { test } => {
				if let Some(time) =
					self.sequence.as_mut()
					    .and_then(|id| {
						    id.map(|seq| seq.elapsed_time).ok()
					    }) {
					let sprite = self.sprite.clone().unwrap();
					test.clone().report(self, &sprite, time);
				} else {
					godot_print!("Sequence ended, printing reports!");
					self.state = State::PrintingReports;
				}
			}
			State::PrintingReports => {
				self.frame_count -= 1;
				if self.frame_count > 0 {
					return;
				}

				let count = usize::clamp(self.reports.len(), 0, 5);
				if count <= 0 {
					self.state = State::Finished;
					return;
				}

				self.frame_count = 120;

				let to_print =
					self.reports
					    .drain(..count)
					    .map(|r| r.to_string())
					    .collect::<Vec<_>>()
					    .join("\n");

				godot_print!("{to_print}");
			}
			State::Finished => {}
		}
	}
}

#[godot_api]
impl Tester {
	fn reset(&mut self) {
		self.reports.clear();
		let mut sprite = self.sprite.clone().unwrap();
		sprite.kill_bound_tweens();
		sprite.set_position(Vector2::new(0., 0.));
		sprite.set_modulate(Color::from_rgba(0., 1., 1., 1.));
	}

	#[func]
	fn _start_test(&mut self, test: Test) {
		self.reset();
		self.state = State::Running { test };
		let sprite = self.sprite.clone().unwrap();
		self.sequence = Some(test.start(&sprite));
	}
}