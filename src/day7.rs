use std::{cell::RefCell, collections::HashMap, rc::Rc};

const COMMAND: &str = "$";
const CD_COMMAND: &str = "$ cd";
const LS_COMMAND: &str = "$ ls";

pub fn day7() {
    let mut lines = include_str!("day7.txt").lines().skip(1).peekable();

    // let mut cwd: Vec<String> = Vec::new();

    let root_dir = Directory::new("root".to_string());
    let root_ref = RefCell::new(root_dir);
    let root: Rc<RefCell<Directory>> = Rc::new(root_ref); //Rc<RefCell<Directory>> = Rc::new(RefCell::new(Directory::new()));
                                                          // let mut current: Rc<RefCell<Directory>> = Rc::clone(&root);
                                                          // let mut previous: Option<Rc<RefCell<Directory>>> = None;
    let mut stack: Vec<Rc<RefCell<Directory>>> = vec![Rc::clone(&root)];

    let mut total_size = 0;

    while let Some(mut l) = lines.next() {
        if l.starts_with(LS_COMMAND) {
            println!("{}", l);
            while let Some(p) = lines.next() {
                if p.starts_with(COMMAND) {
                    l = p;
                    break;
                }
                if p.starts_with("dir") == false {
                    let split = p.split_whitespace().collect::<Vec<&str>>();
                    if stack.last().unwrap().borrow().files.contains_key(split[1]) == false {
                        let size = split[0].parse::<u32>().unwrap();
                        stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .files
                            .insert(split[1].to_string(), size);
                        total_size += size;
                    }
                } else {
                    let name = p.chars().skip(4).collect::<String>();
                    if stack.last().unwrap().borrow().has_dir(&name[..]) == false {
                        stack.last().unwrap().borrow_mut().add_dir(name);
                    }
                }
                println!("----- {}", p)
            }
            // dbg!(&stack.last().unwrap());
        }
        if l.starts_with(CD_COMMAND) {
            println!("{}", l);
            let arg = l.chars().skip(CD_COMMAND.len() + 1).collect::<String>();
            if arg == "/" {
                println!("back to root");
                stack.clear();
                stack.push(Rc::clone(&root));
            } else if arg == ".." {
                if stack.len() > 1 {
                    stack.pop();
                    println!("went up to:");
                    dbg!(&stack.last().unwrap());
                }
            } else {
                // cwd.push(clean);
                let current = Rc::clone(stack.last().unwrap());
                let b = current.borrow();
                // dbg!(&b.directories);
                let new = b.directories.get(&arg).unwrap();
                // drop(current);
                stack.push(Rc::clone(new));
                println!("moved into:");
                dbg!(&new);
            }

            // let path = cwd.join("/");
            // println!("path: {}", path);
        }
    }

    let r = &root.borrow();

    // dbg!(&root);
    println!("{}", r.get_total_size());
    println!("{}", total_size);

    let mut sum = 0;
    find(&root, &mut sum);
    println!("sum: {}", sum); // 1212225 too low
}

fn find(root: &Rc<RefCell<Directory>>, sum: &mut u32) {
	let borrow = root.borrow();
    if borrow.get_total_size() <= 100000 {
        *sum += borrow.get_total_size();
		println!("adding {}, sum is {}", borrow.name, sum);
    }
    for dir in borrow.directories.values() {
        find(dir, sum);
    }
}

#[derive(Debug)]
struct Directory {
    // parent: Option<Box<Directory>>,
	name: String,
    files: HashMap<String, u32>,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(/*parent: Option<Box<Directory>>*/name: String) -> Directory {
        Directory {
			name: name,
            /*parent: parent,*/ files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    fn has_dir(&self, name: &str) -> bool {
        self.directories.contains_key(name)
    }

    fn add_dir(&mut self, name: String) {
        self.directories
            .insert(name.to_string(), Rc::new(RefCell::new(Directory::new(name))));
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

    // fn files_mut(mut self) -> &'a mut HashMap<String, u32> {
    // 	&mut self.files
    // }
}
