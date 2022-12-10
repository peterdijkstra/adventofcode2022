pub fn day6() {
    let input = include_str!("day6.txt");

    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(4);

    for (i, window) in windows.enumerate() {
        let mut vec = window.to_vec();
        // println!("vec: {:?}", vec);
        vec.sort();
        vec.dedup();
        if vec.len() != 4 {
            continue;
        }
        // println!("first uniques: {}! {:?}", i, vec);
        println!("answer is: {}", i + vec.len());
        break;
    }
}
