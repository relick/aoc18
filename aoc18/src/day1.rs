use std::collections::HashSet;
use itertools::Itertools;

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

// from csa (github.com/CameronAavik)
pub fn solve2b(input_text: &str) -> Result<String, &'static str> {
	let mut changes = Vec::new();
	for freq_str in input_text.lines() {
		changes.push(freq_str.parse::<i64>().unwrap());
	}
	
	let shift: i64 = changes.iter().sum();
	let get_diff = |(&(xi, xf), &(yi, yf))| if shift > 0 { (yf - xf, xi, yf) } else { (yf - xf, yi, xf) };

	Ok(
	changes.iter()
		.scan(0, |state, &x| { *state = *state + x; Some(*state) })
		.take(changes.len())
		.enumerate()
		.map(|(i, v)| (((v % shift) + shift) % shift, (i, v)))
		.into_group_map()
		.iter().map(|(_, v)| {
			let mut new_v = v.clone();
			new_v.sort_by(|(_,a_v), (_, b_v)| a_v.cmp(&b_v));
			new_v.iter().tuple_windows::<(_,_)>().map(get_diff).collect::<Vec<(i64, usize, i64)>>()
		})
		.flat_map(|v| v)
		.min().unwrap().2.to_string()
	)
}