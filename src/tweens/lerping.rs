use std::fmt::Debug;

use crate::internal::*;

#[derive(Debug, Clone, Copy)]
pub enum StepResult {
	Unfinished { accumulated_t: f64 },
	Finished { excess_time: f64 },
}

#[derive(Debug, Clone)]
pub enum LerpMode<T> {
	Absolute { duration: f64, start: Option<T> },
	SpeedBased { speed: f64, t_sum: f64 },
	Relative { duration: f64, origin: T },
}

pub trait SpireLerp: Sized + Clone + FromGodot + ToGodot {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self;
	fn add_relative(current_at_obj: &Self, previous_relative: &Self, new_relative: &Self) -> Self;
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult);
	fn spire_distance(from: &Self, to: &Self) -> f64;
	
	fn relative_origin() -> Self;
}

trait MoveTowards {
	fn move_towards(from: Self, to: Self, abs_move: Self) -> Self;
}

impl MoveTowards for f32 {
	fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
		if from < to {
			Self::min(from + abs_move, to)
		} else {
			Self::max(from - abs_move, to)
		}
	}
}

impl MoveTowards for f64 {
	fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
		if from < to {
			Self::min(from + abs_move, to)
		} else {
			Self::max(from - abs_move, to)
		}
	}
}

impl MoveTowards for i64 {
	fn move_towards(from: Self, to: Self, abs_move: Self) -> Self {
		if from < to {
			Self::min(from + abs_move, to)
		} else {
			Self::max(from - abs_move, to)
		}
	}
}

impl SpireLerp for i64 {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		(*from as f64 + (to - from) as f64 * t).round() as i64
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self { 
		present_at_obj + new_calc - previous_calc
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		
		let remaining_distance = i64::abs(to - from);
		let abs_step = i64::min(remaining_distance, max_step.floor() as i64);
		
		let unused_time = (max_step - abs_step as f64) / speed;
		let value = i64::move_towards(*from, *to, abs_step);
		
		let step_result =
			if max_step >= remaining_distance as f64 {
				StepResult::Finished { excess_time: unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: unused_time }
			};
		
		(value, step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		i64::abs(to - from) as f64
	}
	
	fn relative_origin() -> Self { 0 }
}

impl SpireLerp for f64 {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		from + (to - from) * t
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		present_at_obj + new_calc - previous_calc
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		
		let remaining_distance = f64::abs(to - from);
		let abs_step = f64::min(remaining_distance, max_step);
		
		let unused_time = (max_step - abs_step) / speed;
		let value = f64::move_towards(*from, *to, abs_step);

		let step_result =
			if max_step >= remaining_distance {
				StepResult::Finished { excess_time: unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: unused_time }
			};
		
		(value, step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		f64::abs(to - from)
	}
	
	fn relative_origin() -> Self { 0. }
}

impl SpireLerp for GString {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = from.to_string();
		let to = to.to_string();
		
		let t = f64::clamp(t, 0.0, 1.0);
		
		let from_len = from.chars().count() as i64;
		let to_len = to.chars().count() as i64;
		let new_len_raw= from_len + ((to_len - from_len) as f64 * t).round() as i64;
		let new_len = i64::abs(new_len_raw) as usize;
		
		let mut result = from.chars().collect::<Vec<_>>();
		let chars_to_take = usize::min((to_len as f64 * t).round() as usize, to_len as usize);
		let taken_chars = to.chars().take(chars_to_take).enumerate();
		for (index, char) in taken_chars {
			if result.len() > index {
				result[index] = char;
			} else {
				result.push(char);
			}
		}

		result.into_iter().take(new_len).collect::<String>().into()
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, next_calc: &Self) -> Self {
		let previous_calc = previous_calc.to_string();
		let next_calc = next_calc.to_string();
		
		let delta = {
			let old_count = previous_calc.chars().count();
			let new_count = next_calc.chars().count();

			if old_count >= new_count {
				&previous_calc
			} else {
				let new_index =
					previous_calc
						.char_indices()
						.nth(old_count)
						.map(|(index, _)| index)
						.unwrap_or(0);

				&next_calc[new_index..]
			}
		};
		
		(present_at_obj.to_string() + delta).into()
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		let abs_step = max_step.floor() as i64;
		let unused_time = (max_step - abs_step as f64) / speed;
		
		let from_len = from.len();
		let to_len = to.len();
		
		let to_str =
			to.to_string();

		let mut result = from.to_string().chars().collect::<Vec<_>>();
		
		let mut remaining = abs_step;

		for (idx, char) in to_str.chars().enumerate() {
			if remaining <= 0 {
				break;
			}
			
			if idx >= from_len {
				result.push(char);
				remaining -= 1;
			} else if result[idx] != char {
				result[idx] = char;
				remaining -= 1;
			}
		}
		
		let mut char_delta = from_len as i64 - to_len as i64;
		while char_delta > 0 && remaining > 0 {
			result.pop();
			char_delta -= 1;
			remaining -= 1;
		}
		
		let final_unused_time = unused_time + (remaining as f64 / speed);
		let value = result.into_iter().collect::<String>();
		
		let step_result = 
			if value == to_str {
				StepResult::Finished { excess_time: final_unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: final_unused_time }
			};
		
		(value.into(), step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		let mut distance = 0;
		let from_len = from.len();
		let to_len = to.len();

		let from_chars = from.chars();
		let to_chars = to.chars();
		
		for i in 0..usize::min(from_len, to_len) {
			if from_chars[i] != to_chars[i] {
				distance += 1;
			}
		}
		
		let count_abs = usize::abs_diff(from_len, to_len);
		(distance + count_abs) as f64
	}
	
	fn relative_origin() -> Self { GString::new() }
}

impl SpireLerp for Color {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		let t = t as f32;
		Color::from_rgba(
			from.r + (to.r - from.r) * t,
			from.g + (to.g - from.g) * t,
			from.b + (to.b - from.b) * t,
			from.a + (to.a - from.a) * t)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = *present_at_obj;
		
		Color::from_rgba(
			present_at_obj.r + (new_calc.r - previous_calc.r),
			present_at_obj.g + (new_calc.g - previous_calc.g),
			present_at_obj.b + (new_calc.b - previous_calc.b),
			present_at_obj.a + (new_calc.a - previous_calc.a))
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		let max_distance = [
			(from.r - to.r).abs(),
			(from.g - to.g).abs(),
			(from.b - to.b).abs(),
			(from.a - to.a).abs()
		].into_iter()
		 .max_by(f32::total_cmp)
		 .unwrap_or(0.);
		
		let abs_step = f32::min(max_distance, max_step as f32);
		
		let unused_time = (max_step - abs_step as f64) / speed;
		let value = Color::from_rgba(
			f32::move_towards(from.r, to.r, abs_step),
			f32::move_towards(from.g, to.g, abs_step),
			f32::move_towards(from.b, to.b, abs_step),
			f32::move_towards(from.a, to.a, abs_step));

		let step_result =
			if abs_step >= max_distance {
				StepResult::Finished { excess_time: unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: unused_time }
			};
		
		(value, step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		[
			(from.r - to.r).abs(), 
			(from.g - to.g).abs(), 
			(from.b - to.b).abs(),
			(from.a - to.a).abs()
		].into_iter()
		 .max_by(f32::total_cmp)
		 .unwrap_or(0.) as f64
	}
	
	fn relative_origin() -> Self { Color::from_rgba(0., 0., 0., 0.) }
}

impl SpireLerp for Vector2 {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = *from;
		let to = *to;
		
		Vector2::new(
			from.x + (to.x - from.x) * t as f32,
			from.y + (to.y - from.y) * t as f32)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = *present_at_obj;
		
		Vector2::new(
			present_at_obj.x + (new_calc.x - previous_calc.x),
			present_at_obj.y + (new_calc.y - previous_calc.y))
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		let max_distance = [
			(from.x - to.x).abs(),
			(from.y - to.y).abs()
		].into_iter()
		 .max_by(f32::total_cmp)
		 .unwrap_or(0.);
		
		let abs_step = f32::min(max_distance, max_step as f32);
		
		let unused_time = (max_step - abs_step as f64) / speed;
		let value = Vector2::new(
			f32::move_towards(from.x, to.x, abs_step),
			f32::move_towards(from.y, to.y, abs_step));

		let step_result =
			if abs_step >= max_distance {
				StepResult::Finished { excess_time: unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: unused_time }
			};
		
		(value, step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		[(from.x - to.x).abs(), (from.y - to.y).abs()]
			.into_iter()
			.max_by(f32::total_cmp)
			.unwrap_or(0.) as f64
	}
	
	fn relative_origin() -> Self { Vector2::new(0., 0.) }
}

impl SpireLerp for Vector3 {
	fn spire_lerp(from: &Self, to: &Self, t: f64) -> Self {
		let from = *from;
		let to = *to;

		Vector3::new(
			from.x + (to.x - from.x) * t as f32,
			from.y + (to.y - from.y) * t as f32,
			from.z + (to.z - from.z) * t as f32)
	}

	fn add_relative(present_at_obj: &Self, previous_calc: &Self, new_calc: &Self) -> Self {
		let present_at_obj = *present_at_obj;

		Vector3::new(
			present_at_obj.x + (new_calc.x - previous_calc.x),
			present_at_obj.y + (new_calc.y - previous_calc.y),
			present_at_obj.z + (new_calc.z - previous_calc.z))
	}
	
	fn spire_step(from: &Self, to: &Self, speed: f64, t: f64) -> (Self, StepResult) {
		let max_step = speed * t;
		let max_distance = [
			(from.x - to.x).abs(),
			(from.y - to.y).abs(),
			(from.z - to.z).abs()
		].into_iter()
		 .max_by(f32::total_cmp)
		 .unwrap_or(0.);
		
		let abs_step = f32::min(max_distance, max_step as f32);
		
		let unused_time = (max_step - abs_step as f64) / speed;
		let value = Vector3::new(
			f32::move_towards(from.x, to.x, abs_step),
			f32::move_towards(from.y, to.y, abs_step),
			f32::move_towards(from.z, to.z, abs_step));

		let step_result =
			if abs_step >= max_distance {
				StepResult::Finished { excess_time: unused_time }
			} else {
				StepResult::Unfinished { accumulated_t: unused_time }
			};
		
		(value, step_result)
	}
	
	fn spire_distance(from: &Self, to: &Self) -> f64 {
		[
			(from.x - to.x).abs(),
			(from.y - to.y).abs(),
			(from.z - to.z).abs()
		].into_iter()
		 .max_by(f32::total_cmp)
		 .unwrap_or(0.) as f64
	}
	
	fn relative_origin() -> Self { Vector3::new(0., 0., 0.) }
}
