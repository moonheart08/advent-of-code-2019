use std::str::FromStr;

pub fn solve_fuel(input: f32) -> f32 {
	(input / 3.0).floor() - 2.0
}

pub fn solve_fuel_recursive(input: f32) -> f32 {
	let mut total = 0.0;
	let mut wv = input; // working value
	loop {
		let r = solve_fuel(wv);
		if r > 0.0 {
			wv = r;
			total += r;
		} else {
			break;
		}
	}
	return total;
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i64 {
	input.split('\n').map(|inp| f32::from_str(inp).unwrap()).map(|inp| solve_fuel(inp) as i64).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i64 {
	input.split('\n').map(|inp| f32::from_str(inp).unwrap()).map(|inp| solve_fuel_recursive(inp) as i64).sum()

}
