use regex::Regex;
use std::collections::HashMap;

fn change_date(month: &str, day: &str, hour: &str) -> usize {
	let result;
	let month: usize = month.parse().unwrap();
	let day: usize = day.parse().unwrap();
	if hour == "23" {
		if day == 28 && month == 2 {
			result = 301;
		} else if (day == 30 && (month == 4 || month == 6 || month == 9 || month == 11)) || day == 31 {
			result = 1 + (100 * (month + 1));
		} else {
			result = day + 1 + (month * 100);
		}
	} else {
		result = day + (100 * month);
	}
	result
}

fn parse_text(input_text: &str) -> (Vec<usize>, HashMap<usize, (usize, Vec<usize>)>, Vec<(usize, usize, bool)>) {
	//regex
	let guard_re = Regex::new(r"\[1518\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (?:Guard #(\d+) begins shift)").unwrap();
	let action_re = Regex::new(r"\[1518\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] (falls asleep|wakes up)").unwrap();

	let mut dates_to_guards = vec![0usize;1232];
	let guards = HashMap::new();
	
	for cap in guard_re.captures_iter(input_text) {
		let id: usize = cap[5].parse().unwrap();
		dates_to_guards[change_date(&cap[1], &cap[2], &cap[3])] = id;
	}

	let mut guard_actions = Vec::new();
	for cap in action_re.captures_iter(input_text) {
		let month: usize = cap[1].parse().unwrap();
		let day: usize = cap[2].parse().unwrap();
		let minute: usize = cap[4].parse().unwrap();
		guard_actions.push(((month * 100) + day, minute, &cap[5] == "falls asleep"));
	}

	guard_actions.sort_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

	(dates_to_guards, guards, guard_actions)
}

fn guards_insert() -> (usize, Vec<usize>) {
	(0usize, vec![0usize;60])
}

pub fn solve1(input_text: &str) -> String {
	let mut parsed = parse_text(input_text);
	let dates_to_guards = parsed.0;
	let guards = &mut parsed.1;
	let guard_actions = parsed.2;

	let mut prev_sleep = 0;
	let mut max_sleep = 0;
	let mut max_guard_id = 0;
	for (date, minutes, action) in guard_actions.iter() {
		if *action {
			prev_sleep = *minutes;
		} else {
			guards.entry(dates_to_guards[*date]).or_insert_with(guards_insert).0 += *minutes - prev_sleep;
			if max_sleep < guards[&dates_to_guards[*date]].0 {
				max_sleep = guards[&dates_to_guards[*date]].0;
				max_guard_id = dates_to_guards[*date];
			}
			for i in prev_sleep..*minutes {
				guards.entry(dates_to_guards[*date]).or_insert_with(guards_insert).1[i] += 1;
			}
		}
	}

	let mut max_min = 0;
	let mut max_min_time = 0;
	for i in 0..60 {
		if max_min < guards[&max_guard_id].1[i] {
			max_min = guards[&max_guard_id].1[i];
			max_min_time = i;
		}
	}
	(max_guard_id * max_min_time).to_string()
}

pub fn solve2(input_text: &str) -> String {
	let mut parsed = parse_text(input_text);
	let dates_to_guards = parsed.0;
	let guards = &mut parsed.1;
	let guard_actions = parsed.2;

	let mut prev_sleep = 0;
	let mut max_sleep = 0;
	let mut max_guard_id = 0;
	let mut minute = 0;
	for (date, minutes, action) in guard_actions.iter() {
		if *action {
			prev_sleep = *minutes;
		} else {
			guards.entry(dates_to_guards[*date]).or_insert_with(guards_insert).0 += *minutes - prev_sleep;
			for i in prev_sleep..*minutes {
				guards.entry(dates_to_guards[*date]).or_insert_with(guards_insert).1[i] += 1;
				if max_sleep < guards[&dates_to_guards[*date]].1[i] {
					max_sleep = guards[&dates_to_guards[*date]].1[i];
					max_guard_id = dates_to_guards[*date];
					minute = i;
				}
			}
		}
	}

	(max_guard_id * minute).to_string()
}