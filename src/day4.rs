pub fn day4() {
	let lines = include_str!("day4.txt").lines();

	let mut contains = 0;

	let mut overlaps = 0;

	for line in lines {
		let split: Vec<&str> = line.split(',').collect();
		let range_a_values: Vec<u32> = split[0].split('-').map(|x| x.parse::<u32>().unwrap()).collect();
		let range_b_values: Vec<u32> = split[1].split('-').map(|x| x.parse::<u32>().unwrap()).collect();
		
		if range_a_values[0] >= range_b_values[0] && range_a_values[1] <= range_b_values[1] ||
		range_b_values[0] >= range_a_values[0] && range_b_values[1] <= range_a_values[1] {
			contains += 1;
		}

		if range_a_values[0] <= range_b_values[1] && range_b_values[0] <= range_a_values[1] {
			overlaps += 1;
		}
	}

	println!("CONTAINS: {}", contains);
	println!("OVERLAPS: {}", overlaps);
}