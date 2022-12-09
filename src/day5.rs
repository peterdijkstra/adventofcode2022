pub fn day5() {
    let lines = include_str!("day5.txt").lines();

    let stack_data: Vec<&str> = lines.clone().take_while(|l| l.len() != 0).collect();

    for data in &stack_data {
        println!("{}", data);
    }

    let move_data: Vec<&str> = lines.skip_while(|l| l.len() != 0).skip(1).collect();

    // for data in &move_data {
    // 	println!("{}", data);
    // }

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut stack_data_iter = stack_data.iter().map(|l| 
		l.chars().skip(1).step_by(2).step_by(2).collect::<String>());

    let last = stack_data_iter.next_back().unwrap();

	dbg!(last);

    for line in stack_data_iter {
        for (i, c) in line.chars().enumerate() {
            let mut stack = stacks.get_mut(i);
            if stack.is_none() {
                stacks.push(Vec::new());
                stack = stacks.get_mut(i);
            }
            if c != ' ' {
                stack.unwrap().insert(0, c);
            }
        }
    }

    for (i, s) in stacks.iter().enumerate() {
        println!("STACK {}: {}", i, s.iter().collect::<String>());
    }

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

        for _ in 0..split[0] {
            let mut take_char: Option<char> = None;

            {
                let take = stacks.get_mut(from).unwrap();
                take_char = Some(take.pop().unwrap());
            }

            if let Some(c) = take_char {
                let push = stacks.get_mut(to).unwrap();
                push.push(c);
            }
        }

        // for (i, s) in stacks.iter().enumerate() {
        // 	println!("STACK {}: {}", i, s.iter().collect::<String>());
        // }
    }

    let answer = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("ANSWER: {}", answer); // should be TWSGQHNHL
}
