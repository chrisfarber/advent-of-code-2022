use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() {
    let map = read_tree_map("inputs/day8.txt");
    let width = map.0;

    let mut visibility = vec![0; width * width];

    println!("the width is {}", map.0);
    for dir in [
        Direction::Left,
        Direction::Down,
        Direction::Right,
        Direction::Up,
    ] {
        scan_map(&map, dir, |(y, x), last, cur| {
            println!("y: {}, x: {}, last: {:?}, cur: {}", y, x, last, cur);
            if let Some(last) = last {
                if last >= cur {
                    return last;
                }
            }
            visibility[width * x + y] = 1;
            cur
        });
    }

    let visible: i32 = visibility.iter().sum();
    println!("# of visible trees: {}", visible);
}

enum Direction {
    Left,
    Right,
    Down,
    Up,
}
type Map = (usize, Vec<Vec<u8>>);

fn scan_map<F>(map: &Map, direction: Direction, mut cb: F)
where
    F: FnMut((usize, usize), Option<u8>, u8) -> u8,
{
    let width = map.0;
    let (flip_scan, flip_access) = match direction {
        Direction::Right => (false, false),
        Direction::Down => (false, true),
        Direction::Up => (true, true),
        Direction::Left => (true, false),
    };
    let indexes = if flip_access {
        |a, b| (b, a)
    } else {
        |a, b| (a, b)
    };
    for row in 0..map.0 {
        let mut cur = None;
        let scan: Box<dyn Iterator<Item = usize>> = if flip_scan {
            Box::new((0..width).into_iter().rev())
        } else {
            Box::new((0..width).into_iter())
        };
        for scanned in scan {
            println!("scanned {:?}", scanned);
            let (x, y) = indexes(row, scanned);
            let visiting = map.1[x][y];
            cur = Some(cb((y, x), cur, visiting));
        }
    }
}

fn read_tree_map(from: &str) -> (usize, Vec<Vec<u8>>) {
    let f = File::open(from).unwrap();
    let br = BufReader::new(f);
    let mut map: Vec<Vec<u8>> = vec![];
    for line in br.lines() {
        map.push(line.unwrap().bytes().map(|b| b - b'0').collect());
    }

    let y_height = map.len();
    let x_heights: HashSet<usize> = map.iter().map(|r| r.len()).collect();
    if x_heights.len() != 1 || !x_heights.contains(&y_height) {
        panic!("The map did not have the sime height and width.");
    }
    (y_height, map)
}
