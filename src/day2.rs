pub fn day2() {
    let lines = include_str!("day2.txt").lines();

    let mut part_1_score = 0;
    let mut part_2_score = 0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        let theirs_char = split[0].chars().next().unwrap();
        let mine_original_char = split[1].chars().next().unwrap();
        let mine_shifted = mine_original_char as u32 - 23;
        let mine_char = char::from_u32(mine_shifted).unwrap();

        let theirs = get_value(theirs_char);
        let mine = get_value(mine_char);

        part_1_score += get_round_score(&theirs, &mine);

        let requested_result = get_result(mine_original_char);
        let requested_answer = get_required_answer(&theirs, &requested_result);
        part_2_score += get_round_score(&theirs, &requested_answer);
    }

    println!("PART 1 SCORE: {}", part_1_score);
    println!("PART 2 SCORE: {}", part_2_score);
}

fn get_round_score(theirs: &Value, mine: &Value) -> u32 {
    let mut score = 0;
    score += get_battle_score(&theirs, &mine);
    score += get_usage_score(&mine);
    score
}

fn get_value(input: char) -> Value {
    match input {
        'A' => Value::Rock,
        'B' => Value::Paper,
        'C' => Value::Scissors,
        _ => panic!(),
    }
}

fn get_battle_score(theirs: &Value, mine: &Value) -> u32 {
    match theirs {
        Value::Rock => match mine {
            Value::Rock => 3,
            Value::Paper => 6,
            Value::Scissors => 0,
        },
        Value::Paper => match mine {
            Value::Rock => 0,
            Value::Paper => 3,
            Value::Scissors => 6,
        },
        Value::Scissors => match mine {
            Value::Rock => 6,
            Value::Paper => 0,
            Value::Scissors => 3,
        },
    }
}

fn get_usage_score(mine: &Value) -> u32 {
    match mine {
        Value::Rock => 1,
        Value::Paper => 2,
        Value::Scissors => 3,
    }
}

#[derive(Debug)]
enum Value {
    Rock,
    Paper,
    Scissors,
}

fn get_result(input: char) -> Result {
    match input {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => panic!(),
    }
}

fn get_required_answer(theirs: &Value, result: &Result) -> Value
{
    match theirs {
        Value::Rock => match result {
            Result::Lose => Value::Scissors,
            Result::Draw => Value::Rock,
            Result::Win => Value::Paper
        }
        Value::Paper => match result {
            Result::Lose => Value::Rock,
            Result::Draw => Value::Paper,
            Result::Win => Value::Scissors
        }
        Value::Scissors => match result {
            Result::Lose => Value::Paper,
            Result::Draw => Value::Scissors,
            Result::Win => Value::Rock
        }
    }
}

#[derive(Debug)]
enum Result {
    Lose,
    Draw,
    Win,
}