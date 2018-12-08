use regex::Regex;

fn get_sum_metadata(input_nums: &Vec<u64>, index: usize) -> (u64, usize) {
	let num_nodes = input_nums[index] as usize;
	let num_metadata = input_nums[index + 1] as usize;
	let mut sum = 0;

	let mut updating_index = index + 2;
	for _ in 0..num_nodes {
		let result_tuple = get_sum_metadata(input_nums, updating_index);
		updating_index = result_tuple.1;
		sum += result_tuple.0;
	}

	for _ in 0..num_metadata {
		sum += input_nums[updating_index];
		updating_index += 1;
	}

	(sum, updating_index)
}

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	let num_re = Regex::new(r"(\d+)").unwrap();
	let mut nums = Vec::new();
	for cap in num_re.captures_iter(input_text) {
		nums.push(cap[1].parse::<u64>().unwrap());
	}

	Ok(get_sum_metadata(&nums, 0).0.to_string())
}

fn get_value_of_root(input_nums: &Vec<u64>, index: usize) -> (u64, usize) {
	let num_nodes = input_nums[index] as usize;
	let num_metadata = input_nums[index + 1] as usize;
	let mut value = 0;
	let mut child_values = Vec::new();

	let mut updating_index = index + 2;
	for _ in 0..num_nodes {
		let result_tuple = get_value_of_root(input_nums, updating_index);
		updating_index = result_tuple.1;
		child_values.push(result_tuple.0);
	}

	for _ in 0..num_metadata {
		if child_values.len() > 0 {
			if (input_nums[updating_index] as usize) <= child_values.len() {
				value += child_values[(input_nums[updating_index] as usize) - 1];
			}
		} else {
			value += input_nums[updating_index];
		}
		updating_index += 1;
	}

	(value, updating_index)
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	let num_re = Regex::new(r"(\d+)").unwrap();
	let mut nums = Vec::new();
	for cap in num_re.captures_iter(input_text) {
		nums.push(cap[1].parse::<u64>().unwrap());
	}

	Ok(get_value_of_root(&nums, 0).0.to_string())
}