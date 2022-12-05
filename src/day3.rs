pub fn day3() {
    let lines = include_str!("day3.txt").lines();
    let mut sum = 0;
    for line in lines {
        let split = line.split_at(line.len() / 2);
        let vec_a: Vec<char> = split.0.chars().collect();
        let vec_b: Vec<char> = split.1.chars().collect();

        for c_a in vec_a {
            let mut should_break = false;
            for c_b in &vec_b {
                if c_a == *c_b {
                    println!("match {}", c_a);
                    sum += calc_score(&c_a);
                    should_break = true;
                    break;
                }
            }
            if should_break {
                break;
            }
        }
    }
    println!("SUM: {}", sum);
}

fn calc_score(c: &char) -> u32 {
    let is_upper = c.is_uppercase();
    *c as u32 - if is_upper { 38 } else { 96 }
}