use std::collections::VecDeque;

fn swap_case(c: u8) -> u8 {
	let upper = c.to_ascii_uppercase();
	if upper == c {
		return c.to_ascii_lowercase();
	} else {
		return upper;
	}
}

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	let mut polymer: VecDeque<u8> = input_text.trim().bytes().collect();
	let mut i: usize = 0;
	while i < (polymer.len() - 1) {
		while i < (polymer.len() - 1) && swap_case(polymer[i]) == polymer[i+1] {
			polymer.remove(i);
			polymer.remove(i);
			if i != 0 {
				i -= 1;
			}
		}
		i += 1;
	}
	Ok(polymer.len().to_string())
}

// Taken from /r/rust, it's fast and cool
pub fn solve1b(input_text: &str) -> Result<String, &'static str> {
	Ok(input_text.trim().chars().fold(Vec::<char>::new(), |mut stack, c| {
		match stack.last().cloned() {
			Some(v) if v != c && c.to_ascii_uppercase() == v.to_ascii_uppercase() => {
				stack.pop();
			}
			_ => stack.push(c)
		}
		stack
	}).len().to_string())
}

fn solve1b_but_first(input_text: &str) -> String {
	input_text.trim().chars().fold(Vec::<char>::new(), |mut stack, c| {
		match stack.last().cloned() {
			Some(v) if v != c && c.to_ascii_uppercase() == v.to_ascii_uppercase() => {
				stack.pop();
			}
			_ => stack.push(c)
		}
		stack
	}).into_iter().collect()
}

fn solve1b_but_for_solve2(input_text: &str, vanish_c: char) -> usize {
	let upper_c = vanish_c.to_ascii_uppercase();
	input_text.trim().chars().fold(Vec::<char>::new(), |mut stack, c| {
		match stack.last().cloned() {
			Some(v) if v != c && c.to_ascii_uppercase() == v.to_ascii_uppercase() => {
				stack.pop();
			}
			_ => if c != upper_c && c != vanish_c { stack.push(c) }
		}
		stack
	}).len()
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	let char_arr: Vec<char> = "abcdefghijklmnopqrtstuvwxyz".chars().collect();
	let result_1 = solve1b_but_first(input_text);
	let mut min: usize = 50001;
	for c in char_arr {
		let length: usize = solve1b_but_for_solve2(&result_1[..], c);
		if length < min {
			min = length;
		}
	}

	Ok(min.to_string())
}