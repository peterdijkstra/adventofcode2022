pub fn day2() {
    let lines = include_str!("day2.txt").lines();

    let mut score = 0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        
        let theirs_char = split[0].chars().next().unwrap();
        let mine_shifted = split[1].chars().next().unwrap() as u32 - 23;
        let mine_char = char::from_u32(mine_shifted).unwrap();

        let theirs = get_value(theirs_char);
        let mine = get_value(mine_char);

        score += get_battle_score(&theirs, &mine);
        score += get_usage_score(&mine);
    }

    println!("SCORE: {}", score);
}

fn get_value(input: char) -> Value
{
    match input {
        'A' => Value::Rock,
        'B' => Value::Paper,
        'C' => Value::Scissors,
        _ => panic!()
    }
}

fn get_battle_score(theirs: &Value, mine: &Value) -> u32
{
    match theirs {
        Value::Rock => match mine {
            Value::Rock => 3,
            Value::Paper => 6,
            Value::Scissors => 0
        }
        Value::Paper => match mine {
            Value::Rock => 0,
            Value::Paper => 3,
            Value::Scissors => 6
        }
        Value::Scissors => match mine {
            Value::Rock => 6,
            Value::Paper => 0,
            Value::Scissors => 3,
        }
    }
}

fn get_usage_score(mine: &Value) -> u32
{
    match mine {
        Value::Rock => 1,
        Value::Paper => 2,
        Value::Scissors => 3,
    }
}

#[derive(Debug)] 
enum Value {
    Rock, Paper, Scissors
}