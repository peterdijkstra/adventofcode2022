pub fn day1() {
    let lines = include_str!("day1.txt").lines();
    let mut calories = vec![1i32, 0];
    let mut calorie_iter = 0;
    for line in lines {
        if line.is_empty() {
            calorie_iter += 1;
            calories.push(0);
        } else {
            let calorie_amount = line.parse::<i32>().unwrap();
            calories[calorie_iter] += calorie_amount;
        }
    }

    let mut highest = 0;
    for c in calories.clone() {
        if c > highest {
            highest = c;
        }
    }

    println!("HIGHEST: {}", highest);

    // part 2

    calories.sort();
    calories.reverse();

    println!(
        "THREE HIGHEST COMBINED: {}",
        calories[0] + calories[1] + calories[2]
    );
}
