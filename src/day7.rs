use std::{cell::RefCell, collections::HashMap, rc::Rc};

const COMMAND: &str = "$";
const CD_COMMAND: &str = "$ cd";
const LS_COMMAND: &str = "$ ls";

pub fn day7() {
    let mut lines = include_str!("day7.txt").lines().skip(1).peekable();

    let root_dir = Directory::new("root".to_string());
    let root_ref = RefCell::new(root_dir);
    let root: Rc<RefCell<Directory>> = Rc::new(root_ref);
    let mut stack: Vec<Rc<RefCell<Directory>>> = vec![Rc::clone(&root)];

    while let Some(mut l) = lines.next() {
        if l.starts_with(LS_COMMAND) {
            while let Some(p) = lines.next() {
                if p.starts_with(COMMAND) {
                    l = p;
                    break;
                }
                if p.starts_with("dir") == false {
                    let split = p.split_whitespace().collect::<Vec<&str>>();
                    if stack.last().unwrap().borrow().files.contains_key(split[1]) == false {
                        let name = split[1].to_string();
                        let size = split[0].parse::<u32>().unwrap();
                        stack.last().unwrap().borrow_mut().files.insert(name, size);
                    }
                } else {
                    let name = p.chars().skip(4).collect::<String>();
                    if stack.last().unwrap().borrow().has_dir(&name[..]) == false {
                        stack.last().unwrap().borrow_mut().add_dir(name);
                    }
                }
            }
        }
        if l.starts_with(CD_COMMAND) {
            let arg = l.chars().skip(CD_COMMAND.len() + 1).collect::<String>();
            if arg == "/" {
                stack.clear();
                stack.push(Rc::clone(&root));
            } else if arg == ".." {
                if stack.len() > 1 {
                    stack.pop();
                }
            } else {
                let current = Rc::clone(stack.last().unwrap());
                let borrow = current.borrow();
                let dir = borrow.directories.get(&arg).unwrap();
                stack.push(Rc::clone(dir));
            }
        }
    }

    let r = &root.borrow();

    let mut sum = 0;
    calc_part_1(&root, &mut sum);
    println!("part 1 answer: {}", sum); // 1432936

	let total_space = 70000000;
	let required_space = 30000000;
	let current_space = total_space - r.get_total_size();
	let missing_space= required_space - current_space;

	let mut smallest = u32::MAX;
	calc_part_2(&root, &mut smallest, missing_space);
	println!("part 2 answer: {}", smallest); // 272298
}

fn calc_part_1(root: &Rc<RefCell<Directory>>, sum: &mut u32) {
    let borrow = root.borrow();
    if borrow.get_total_size() <= 100000 {
        *sum += borrow.get_total_size();
    }
    for dir in borrow.directories.values() {
        calc_part_1(dir, sum);
    }
}

fn calc_part_2(root: &Rc<RefCell<Directory>>, smallest: &mut u32, missing_space: u32)
{
	let borrow = root.borrow();
	let total_size = borrow.get_total_size();
	if total_size >= missing_space && total_size < *smallest {
		*smallest = total_size;
	}
	for dir in borrow.directories.values() {
        calc_part_2(dir, smallest, missing_space);
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, u32>,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    fn has_dir(&self, name: &str) -> bool {
        self.directories.contains_key(name)
    }

    fn add_dir(&mut self, name: String) {
        self.directories.insert(
            name.to_string(),
            Rc::new(RefCell::new(Directory::new(name))),
        );
    }

    fn get_file_size(&self) -> u32 {
        self.files.values().sum()
    }

    fn get_total_size(&self) -> u32 {
        let mut size = 0;
        for dir in self.directories.values() {
            size += dir.borrow().get_total_size();
        }

        size + self.get_file_size()
    }
}
