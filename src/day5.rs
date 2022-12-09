pub fn day5() {
    let lines = include_str!("day5.txt").lines();

    let stack_data: Vec<&str> = lines.clone().take_while(|l| l.len() != 0).collect();

    // for data in &stack_data {
    //     println!("{}", data);
    // }

    let move_data: Vec<&str> = lines.skip_while(|l| l.len() != 0).skip(1).collect();

    // for data in &move_data {
    // 	println!("{}", data);
    // }

    let mut stack_data_iter = stack_data
        .iter()
        .map(|l| l.chars().skip(1).step_by(2).step_by(2).collect::<String>());

    let last = stack_data_iter.next_back().unwrap();

    let stack_amount = last.chars().last().unwrap().to_digit(10).unwrap() as usize;
    let mut stacks_1: Vec<Vec<char>> = vec![Vec::new(); stack_amount];

    for line in stack_data_iter {
        for (i, c) in line.chars().enumerate() {
            let stack = stacks_1.get_mut(i);
            if c.is_whitespace() == false {
                stack.unwrap().insert(0, c);
            }
        }
    }

    // for (i, s) in stacks_1.iter().enumerate() {
    //     println!("STACK {}: {}", i, s.iter().collect::<String>());
    // }

    let mut stacks_2 = stacks_1.clone();

    for line in move_data {
        let nums = line
            .chars()
            .filter(|c| c.is_numeric() || c.is_whitespace())
            .collect::<String>();
        let split: Vec<_> = nums
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        // println!("{:?}", split);
        // amount, from, to
        let from = split[1] - 1;
        let to = split[2] - 1;

        // part 1
        {
            for _ in 0..split[0] {
                let mut take_char: Option<char> = None;
                {
                    let take = stacks_1.get_mut(from).unwrap();
                    take_char = Some(take.pop().unwrap());
                }

                if let Some(c) = take_char {
                    let push = stacks_1.get_mut(to).unwrap();
                    push.push(c);
                }
            }
        }

        // part 2
        {
            let amount = split[0];
            let take = stacks_2.get_mut(from).unwrap();
            // dbg!(&take);
            let mut splice = take
                .splice(take.len() - amount..take.len(), vec![])
                .collect::<Vec<char>>();
            stacks_2.get_mut(to).unwrap().append(&mut splice);
        }
    }

    let answer_1 = stacks_1
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    println!("ANSWER 1: {}", answer_1); // should be TWSGQHNHL

    let answer_2 = stacks_2
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    println!("ANSWER 2: {}", answer_2); // should be JNRSCDWPP
}
