use std::collections::{HashMap};

pub fn day3() {
    let lines: Vec<&str> = include_str!("day3.txt").lines().collect();
    let mut sum = 0;
    for line in &lines {
        let split = line.split_at(line.len() / 2);
        let vec_a: Vec<char> = split.0.chars().collect();
        let vec_b: Vec<char> = split.1.chars().collect();

        for c_a in vec_a {
            let mut should_break = false;
            for c_b in &vec_b {
                if c_a == *c_b {
                    // println!("match {}", c_a);
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

    let mut sum_2 = 0;

    for line in lines.chunks(3) {
        let l = line.to_vec();
        // println!("{}", "\nnew chunk!");
        let mut group_map: HashMap<char, u8> = HashMap::new();
        for w in l {
            let string = w;
            let mut chars: Vec<char> = string.chars().collect();
            chars.sort();
            chars.dedup();
            // let uniq: String = chars.iter().collect();
            // println!("deduped: {}", uniq);
            for c in chars {
                if group_map.contains_key(&c) {
                    if let Some(x) = group_map.get_mut(&c) {
                        *x += 1;
                    }
                    continue;
                }
                group_map.insert(c, 0);
            }
        }

        for kv in group_map {
            if kv.1 == 2 {
                // println!("common: {}", kv.0);
                sum_2 += calc_score(&kv.0);
            }
        }
    }

    println!("SUM 2: {}", sum_2);
}

fn calc_score(c: &char) -> u32 {
    *c as u32 - if c.is_uppercase() { 38 } else { 96 }
}
