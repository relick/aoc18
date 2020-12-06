use regex::Regex;

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	//regex
	let re = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
	let mut data: Vec<(u32, u32, u32, u32)> = Vec::new();
	for cap in re.captures_iter(input_text) {
		data.push((cap[2].parse().unwrap(), cap[3].parse().unwrap(), cap[4].parse().unwrap(), cap[5].parse().unwrap()));
	}

	//grid
	let mut grid_dat = vec![0u32; 1000 * 1000];

	let mut grid_base: Vec<_> = grid_dat.as_mut_slice().chunks_mut(1000).collect();

	let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

	let mut total: u64 = 0;

	for box_dat in data.iter() {
		for i in (box_dat.0)..(box_dat.0 + box_dat.2) {
			for j in (box_dat.1)..(box_dat.1 + box_dat.3) {
				grid[i as usize][j as usize] += 1;
				if grid[i as usize][j as usize] == 2 {
					total += 1;
				}
			}
		}
	}

	Ok(total.to_string())
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	//regex
	let re = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
	let mut data: Vec<(u32, u32, u32, u32, bool)> = Vec::new();
	for cap in re.captures_iter(input_text) {
		data.push((cap[2].parse().unwrap(), cap[3].parse().unwrap(), cap[4].parse().unwrap(), cap[5].parse().unwrap(), false));
	}

	//grid
	let mut grid_dat = vec![0u32; 1000 * 1000];
	let mut grid_base: Vec<_> = grid_dat.as_mut_slice().chunks_mut(1000).collect();
	let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

	let mut lazy_list = vec![false; data.len()];

	for (id, box_dat) in data.iter().enumerate() {
		for i in (box_dat.0)..(box_dat.0 + box_dat.2) {
			for j in (box_dat.1)..(box_dat.1 + box_dat.3) {
				let ind_i = i as usize;
				let ind_j = j as usize;
				if grid[ind_i][ind_j] != 0 {
					lazy_list[id] = true;
					lazy_list[(grid[ind_i][ind_j] as usize) - 1] = true;
				}
				grid[ind_i][ind_j] = (id as u32) + 1;
			}
		}
	}

	for (id, d) in lazy_list.iter().enumerate() {
		if !*d {
			return Ok((id + 1).to_string());
		}
	}

	Err("Failed to find a result for Day 3 part 2")
}