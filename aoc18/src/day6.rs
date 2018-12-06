use regex::Regex;

fn dist((x,y): (usize, usize), (a,b): (i64, i64)) -> i64 {
	(x as i64 - a).abs() + (y as i64 - b).abs()
}

pub fn solve1(input_text: &str) -> Result<String, &'static str> {
	let coord_re = Regex::new(r"(\d+), (\d+)").unwrap();

	let mut points: Vec<(i64, i64)> = Vec::new();
	for cap in coord_re.captures_iter(input_text) {
		points.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
	}

	let min_x: i64 = points.iter().fold(500, |min, (x,_)| if *x < min { *x } else { min });
	let min_y: i64 = points.iter().fold(500, |min, (_,y)| if *y < min { *y } else { min });

	let points: Vec<(i64, i64)> = points.iter().map(|(x,y)| (x - min_x, y - min_y)).collect();

	let max_x: i64 = points.iter().fold(0, |max, (x,_)| if *x > max { *x } else { max }) + 1;
	let max_y: i64 = points.iter().fold(0, |max, (_,y)| if *y > max { *y } else { max }) + 1;
	let grid_size: usize = if max_x > max_y { max_x as usize } else { max_y as usize };

	let mut grid_dat = vec![((grid_size + grid_size) as i64, -2i64); grid_size * grid_size];
	let mut grid_base: Vec<_> = grid_dat.as_mut_slice().chunks_mut(grid_size).collect();
	let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

	let mut counts = vec![(0i64, true); points.len()];

	for i in 0..grid_size {
		for j in 0..grid_size {
			for (index, p) in points.iter().enumerate() {
				let d = dist((i, j), *p);
				if d < grid[i][j].0 {
					grid[i][j].0 = d;
					if grid[i][j].1 >= 0 { counts[grid[i][j].1 as usize].0 -= 1; }
					grid[i][j].1 = index as i64;
					counts[index].0 += 1;
				} else if d == grid[i][j].0 {
					if grid[i][j].1 >= 0 { counts[grid[i][j].1 as usize].0 -= 1; }
					grid[i][j].1 = -1;
				}
			}
		}
	}

	for (_, ind) in grid[0].iter() {
		if *ind >= 0 { counts[*ind as usize].1 = false; }
	}
	for (_, ind) in grid[grid_size - 1].iter() {
		if *ind >= 0 { counts[*ind as usize].1 = false; }
	}
	for row in grid.iter() {
		if row[0].1 >= 0 { counts[row[0].1 as usize].1 = false; }
		if row[grid_size - 1].1 >= 0 { counts[row[grid_size - 1].1 as usize].1 = false; }
	}

	Ok(counts.iter().filter(|(_, valid)| *valid).map(|(a,_)| a).max().unwrap().to_string())
}

pub fn solve2(input_text: &str) -> Result<String, &'static str> {
	let coord_re = Regex::new(r"(\d+), (\d+)").unwrap();

	let mut points: Vec<(i64, i64)> = Vec::new();
	for cap in coord_re.captures_iter(input_text) {
		points.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
	}

	let min_x: i64 = points.iter().fold(500, |min, (x,_)| if *x < min { *x } else { min });
	let min_y: i64 = points.iter().fold(500, |min, (_,y)| if *y < min { *y } else { min });

	let points: Vec<(i64, i64)> = points.iter().map(|(x,y)| (x - min_x, y - min_y)).collect();

	let max_x: i64 = points.iter().fold(0, |max, (x,_)| if *x > max { *x } else { max }) + 1;
	let max_y: i64 = points.iter().fold(0, |max, (_,y)| if *y > max { *y } else { max }) + 1;
	let grid_size: usize = if max_x > max_y { max_x as usize } else { max_y as usize };

	let mut grid_dat = vec![0i64; grid_size * grid_size];
	let mut grid_base: Vec<_> = grid_dat.as_mut_slice().chunks_mut(grid_size).collect();
	let grid: &mut [&mut [_]] = grid_base.as_mut_slice();

	let mut counts = (grid_size * grid_size) as i64;

	for i in 0..grid_size {
		for j in 0..grid_size {
			for p in points.iter() {
				let d = dist((i, j), *p);
				if grid[i][j] != -1 {
					grid[i][j] += d;
					if grid[i][j] >= 10000 {
						counts -= 1;
						grid[i][j] = -1;
					}
				}
			}
		}
	}

	Ok(counts.to_string())
}