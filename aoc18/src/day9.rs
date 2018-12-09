use std::collections::VecDeque;

fn get_clockwise_insertion(circle_len: usize, current_marble: usize) -> usize {
	((current_marble + 1) % circle_len) + 1
}

fn get_anticlockwise_deletion(circle_len: usize, current_marble: usize) -> usize {
	let mut sub = 7;
	let mut new_spot = current_marble;
	while new_spot < sub {
		sub -= new_spot;
		new_spot = circle_len;
	}
	(new_spot - sub) % circle_len
}

fn solve(num_players: usize, last_marble: usize) -> usize {
	let mut player_scores = vec![0usize; num_players];

	let mut marble_circle = VecDeque::new();
	marble_circle.push_back(0);

	let mut current_marble = 0;
	let mut current_player = 0;

	let mut counter = 0;
	for marble_num in 1..=last_marble {
		counter += 1;
		if counter != 23 {
			current_marble = get_clockwise_insertion(marble_circle.len(), current_marble);
			marble_circle.insert(current_marble, marble_num);
		} else {
			counter = 0;
			player_scores[current_player] += marble_num;
			current_marble = get_anticlockwise_deletion(marble_circle.len(), current_marble);
			player_scores[current_player] += marble_circle[current_marble];
			marble_circle.remove(current_marble);
			if current_marble == marble_circle.len() {
				current_marble = 0;
			}
		}
		current_player += 1;
		if current_player >= num_players {
			current_player = 0;
		}
	}
	*player_scores.iter().max().unwrap()
}

pub fn solve1(_input_text: &str) -> Result<String, &'static str> {
	Ok(solve(411, 72059).to_string())
}

pub fn solve2(_input_text: &str) -> Result<String, &'static str> {
	Ok(solve(411, 7205900).to_string())
}

fn rotate(circle: &mut VecDeque<usize>, amount: i64) {
	if amount >= 0 {
		for _ in 0..(amount as usize) {
			let temp = circle.pop_back().unwrap();
			circle.push_front(temp);
		}
	} else {
		for _ in 0..(-amount as usize) {
			let temp = circle.pop_front().unwrap();
			circle.push_back(temp);
		}
	}
}

// stolen from Marcus
fn solve_rotate(num_players: usize, last_marble: usize) -> usize {
	let mut player_scores = vec![0usize; num_players];

	let mut marble_circle = VecDeque::new();
	marble_circle.push_back(0);

	let mut counter = 0;
	for marble in 1..=last_marble {
		counter += 1;
		if counter == 23 {
			counter = 0;
			rotate(&mut marble_circle, 7);
			player_scores[marble % num_players] += marble + marble_circle.pop_back().unwrap();
			rotate(&mut marble_circle, -1);
		} else {
			rotate(&mut marble_circle, -1);
			marble_circle.push_back(marble);
		}
	}

	*player_scores.iter().max().unwrap()
}

pub fn solve1b(_input_text: &str) -> Result<String, &'static str> {
	Ok(solve_rotate(411, 72059).to_string())
}

pub fn solve2b(_input_text: &str) -> Result<String, &'static str> {
	Ok(solve_rotate(411, 7205900).to_string())
}