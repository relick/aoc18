use std::collections::HashSet;

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	let mut total: i64 = 0;
	for freq_str in input_text.lines() {
		total += freq_str.parse::<i64>().unwrap();
	}
	Ok(total.to_string())
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	let mut freqs = Vec::new();
	for freq_str in input_text.lines() {
		freqs.push(freq_str.parse::<i64>().unwrap());
	}

	let mut seen = HashSet::new();
	let mut total: i64 = 0;
	let mut iter = freqs.iter().cycle();
	while !seen.contains(&total) {
		seen.insert(total);
		total += iter.next().unwrap();
	}
	Ok(total.to_string())
}