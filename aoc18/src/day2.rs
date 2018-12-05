use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	let mut map = HashMap::new();
	let mut total2: u64 = 0;
	let mut total3: u64 = 0;
	for line in input_text.lines() {
		let mut seen2: bool = false;
		let mut seen3: bool = false;
		for c in line.chars() {
			let counter = map.entry(c).or_insert(0);
			*counter += 1;
		}
		for (_, n) in map.drain() {
			if !seen2 && n == 2 {
				total2 += 1;
				seen2 = true;
			} else if !seen3 && n == 3 {
				total3 += 1;
				seen3 = true;
			}
		}
	}
	Ok((total2 * total3).to_string())
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	let mut seen = HashSet::new();
	for line in input_text.lines() {
		for i in 0..line.len() {
			let mut string = String::from(&line[0..i]);
			string.push_str("_");
			string.push_str(&line[i+1..line.len()]);

			if seen.contains(&string) {
				let mut result = String::from(&line[0..1]);
				result.push_str(&line[i+1..line.len()]);
				return Ok(result);
			}
			seen.insert(string);
		}
	}
	Err("Failed to find a result for Day 2 part 2")
}