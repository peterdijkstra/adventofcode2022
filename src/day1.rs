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

    calories.sort();
    calories.reverse();

    println!("FIRST HIGHEST: {}", calories[0]);
    println!("SECOND HIGHEST: {}", calories[1]);
    println!("THIRD HIGHEST: {}", calories[2]);

    println!(
        "THREE HIGHEST COMBINED: {}",
        calories[0] + calories[1] + calories[2]
    );
}
