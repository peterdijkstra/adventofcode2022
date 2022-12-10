pub fn day6() {
    let input = include_str!("day6.txt");

    let chars = input.chars().collect::<Vec<char>>();

	find_marker(&chars, 4);
	find_marker(&chars, 14);
}

fn find_marker(chars: &Vec<char>, length: usize)
{
	let windows_1 = chars.windows(length);
    for (i, window) in windows_1.enumerate() {
        let mut vec = window.to_vec();
        vec.sort();
        vec.dedup();
        if vec.len() != length {
            continue;
        }
        println!("answer for length {}: {}", length, i + vec.len());
        break;
    }
}