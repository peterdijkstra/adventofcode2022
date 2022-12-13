use std::{
    cell::{Ref, RefCell},
    fmt::Display,
    rc::Rc,
    slice::Iter,
};

pub fn day8() {
    let lines = include_str!("day8.txt").lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut grid: Vec<Vec<Rc<RefCell<Tile>>>> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Rc<RefCell<Tile>>> = Vec::new();
        let chars = line.chars().enumerate();
        for (x, c) in chars {
            let p = c.to_digit(10).unwrap();
            let tile = Tile::new(p, x as isize, y as isize);
            row.push(Rc::new(RefCell::new(tile)));
        }
        grid.push(row);
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let yy = y as isize;
            let xx = x as isize;
            let up = Coord::new(xx, yy - 1);
            let right = Coord::new(xx + 1, yy);
            let down = Coord::new(xx, yy + 1);
            let left = Coord::new(xx - 1, yy);

            if up.y >= 0 {
                let t = &grid[up.y_usize()][up.x_usize()];
                tile.borrow_mut().up = Some(Rc::clone(t));
            }
            if right.x < width as isize {
                let t = &grid[right.y_usize()][right.x_usize()];
                tile.borrow_mut().right = Some(Rc::clone(t));
            }
            if down.y < height as isize {
                let t = &grid[down.y_usize()][down.x_usize()];
                tile.borrow_mut().down = Some(Rc::clone(t));
            }
            if left.x >= 0 {
                let t = &grid[left.y_usize()][left.x_usize()];
                tile.borrow_mut().left = Some(Rc::clone(t));
            }
        }
    }

    let mut part_1_answer = 0;
    for row in &grid {
        for tile in row {
            let visible_check = tile.borrow().is_visible();
            if visible_check != TileVisibility::Invisible {
                part_1_answer += 1;
            }
        }
    }

    println!("part 1 answer: {}", part_1_answer); // 1533

    let mut part_2_answer = 0;
    for row in &grid {
        for tile in row {
            let borrow = tile.borrow();
            if borrow.is_visible() == TileVisibility::Border {
                continue;
            }
            let scenic = tile.borrow().calc_scenic();
            if scenic > part_2_answer {
                part_2_answer = scenic;
            }
        }
    }

    println!("part 2 answer: {}", part_2_answer); // 345744
}

#[derive(Debug)]
struct Tile {
    height: u32,
    coord: Coord,
    up: Option<Rc<RefCell<Tile>>>,
    right: Option<Rc<RefCell<Tile>>>,
    down: Option<Rc<RefCell<Tile>>>,
    left: Option<Rc<RefCell<Tile>>>,
}

#[derive(Debug, PartialEq, Eq)]
enum TileVisibility {
    Border,
    Visible,
    Invisible,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

// https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }
}

impl Tile {
    fn new(height: u32, x: isize, y: isize) -> Tile {
        Tile {
            height,
            coord: Coord::new(x, y),
            up: None,
            right: None,
            down: None,
            left: None,
        }
    }

    fn coord_str(&self) -> String {
        format!("({}, {})", self.coord.x, self.coord.y)
    }

    fn get_tile(&self, direction: &Direction) -> &Option<Rc<RefCell<Tile>>> {
        match direction {
            Direction::Up => &self.up,
            Direction::Right => &self.right,
            Direction::Down => &self.down,
            Direction::Left => &self.left,
        }
    }

    fn is_visible(&self) -> TileVisibility {
        if let None = self.up {
            TileVisibility::Border
        } else if let None = self.right {
            TileVisibility::Border
        } else if let None = self.down {
            TileVisibility::Border
        } else if let None = self.left {
            TileVisibility::Border
        } else {
            fn check(tile: Ref<Tile>, dir: &Direction, visible: &mut bool, height: u32) {
                if tile.height >= height {
                    *visible = false;
                    return;
                }
                if let Some(t) = &tile.get_tile(dir) {
                    let tile_ref = t.as_ref();
                    check(tile_ref.borrow(), dir, visible, height);
                }
            }

            for dir in Direction::iterator() {
                let mut visible = true;
                let tile = &self.get_tile(dir).as_ref().unwrap();
                check(tile.borrow(), dir, &mut visible, self.height);
                if visible == true {
                    return TileVisibility::Visible;
                }
            }

            TileVisibility::Invisible
        }
    }

    fn calc_scenic(&self) -> u32 {
        fn count_dist(tile: Ref<Tile>, dir: &Direction, distance: &mut u32, height: u32) {
            if tile.height >= height {
                return;
            }
            if let Some(t) = &tile.get_tile(dir) {
                let tile_ref = t.as_ref();
                *distance += 1;
                count_dist(tile_ref.borrow(), dir, distance, height);
            }
        }

        let mut scenic = 0;

        for dir in Direction::iterator() {
            let mut distance = 1;
            let tile = &self.get_tile(dir).as_ref().unwrap();
            count_dist(tile.borrow(), dir, &mut distance, self.height);
            if scenic == 0 {
                scenic = distance;
            } else {
                scenic *= distance;
            }
        }

        scenic
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile (height: {}\n	coord: {},\n	up: {},\n	right: {},\n	down: {},\n	left: {})",
            &self.height,
            &self.coord_str(),
            if let Some(t) = &self.up {
                let b = t.borrow_mut();
                b.coord_str()
            } else {
                "None".to_string()
            },
            if let Some(t) = &self.right {
                let b = t.borrow_mut();
                b.coord_str()
            } else {
                "None".to_string()
            },
            if let Some(t) = &self.down {
                let b = t.borrow_mut();
                b.coord_str()
            } else {
                "None".to_string()
            },
            if let Some(t) = &self.left {
                let b = t.borrow_mut();
                b.coord_str()
            } else {
                "None".to_string()
            }
        )
    }
}

#[derive(Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn x_usize(&self) -> usize {
        self.x as usize
    }

    fn y_usize(&self) -> usize {
        self.y as usize
    }
}
