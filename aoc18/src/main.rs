extern crate regex;
extern crate itertools;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::fs;
use std::time::Instant;

fn main() {
	let data = fs::read_to_string("01.txt").expect("Unable to read file");
	println!("--- Day 1 ---");
    timeit(" 1", &day1::solve1, &data[..]);
    timeit(" 2", &day1::solve2, &data[..]);
    timeit("2b", &day1::solve2b, &data[..]);
    
	let data = fs::read_to_string("02.txt").expect("Unable to read file");
	println!("\n--- Day 2 ---");
    timeit(" 1", &day2::solve1, &data[..]);
    timeit(" 2", &day2::solve2, &data[..]);

	let data = fs::read_to_string("03.txt").expect("Unable to read file");
	println!("\n--- Day 3 ---");
    timeit(" 1", &day3::solve1, &data[..]);
    timeit(" 2", &day3::solve2, &data[..]);

	let data = fs::read_to_string("04.txt").expect("Unable to read file");
	println!("\n--- Day 4 ---");
    timeit(" 1", &day4::solve1, &data[..]);
    timeit(" 2", &day4::solve2, &data[..]);

	let data = fs::read_to_string("05.txt").expect("Unable to read file");
	println!("\n--- Day 5 ---");
    timeit(" 1", &day5::solve1, &data[..]);
    timeit("1b", &day5::solve1b, &data[..]);
    timeit(" 2", &day5::solve2, &data[..]);
}

fn timeit(part: &str, solver: &Fn(&str) -> Result<String, &'static str>, input: &str) {
    let start = Instant::now();
    let result = match solver(input) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    let duration = start.elapsed();
    println!(
        "Part {}: '{}',     {}ms",
		part,
		result,
        (duration.as_secs() as f64 * 1000 as f64) + (duration.subsec_micros() as f64 / 1000 as f64)
    );
}