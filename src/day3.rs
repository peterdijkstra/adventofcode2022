extern crate array_tool;
use array_tool::vec::*;

pub fn day3() {
    let lines = include_str!("day3.txt").lines();
	let mut sum = 0;
    for line in lines {
        let split = line.split_at(line.len() / 2);
        let vec_a: Vec<char> = split.0.chars().collect();
        let vec_b: Vec<char> = split.1.chars().collect();
        let intersect = vec_a.intersect(vec_b);
        let c = intersect[0];
        let is_upper = c.is_uppercase();
        let value = c as u32 - if is_upper { 38 } else { 96 };
		sum += value;
    }
	println!("SUM: {}", sum);
}
