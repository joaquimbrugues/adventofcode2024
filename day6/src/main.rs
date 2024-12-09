use std::{env,fs,process};
use std::collections::{HashSet,VecDeque,};

#[derive(Debug,Clone,Copy,Hash,PartialEq,Eq)]
enum Dir { N, S, W, E, }

impl Dir {
    fn next(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }

    fn walk(&self, (y, x): &(isize, isize)) -> (isize, isize) {
        match self {
            Self::N => (y - 1, *x),
            Self::E => (*y, x + 1),
            Self::S => (y + 1, *x),
            Self::W => (*y, x - 1),
        }
    }
}

fn read_input(input: &str) -> (((isize, isize), Dir), HashSet<(isize, isize)>) {
    // Read input
    let mut guard = None;
    let cap = input.lines().map(|line| line.chars().count()).sum();
    let mut walkable = HashSet::with_capacity(cap);
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            match c {
                '#' => {},
                '.' => { walkable.insert((y,x)); },
                '^' => {
                    guard = Some(((y,x), Dir::N));
                    walkable.insert((y,x));
                },
                _ => unreachable!(),
            }
        }
    }
    if let Some(guard) = guard {
        (guard, walkable)
    } else {
        panic!("Guard not found in the input!");
    }
}

fn run1(input: &str) -> usize {
    let (mut guard, walkable) = read_input(input);
    let width = *walkable.iter().map(|(_, x)| x).max().unwrap();
    let height = *walkable.iter().map(|(y, _)| y).max().unwrap();
    let out_of_bounds = |pos: &(isize, isize)| pos.0 < 0 || pos.0 > height || pos.1 < 0 || pos.1 > width;

    // Walk
    let mut visited = HashSet::with_capacity(walkable.len());
    loop {
        visited.insert(guard.0);
        let next = guard.1.walk(&guard.0);
        if walkable.contains(&next) {
            guard.0 = next;
        } else {
            if out_of_bounds(&next) {
                break;
            } else {
                guard.1 = guard.1.next();
            }
        }
    }

    visited.len()
}

fn run2(input: &str) -> usize {
    // Read input
    let (mut guard, walkable) = read_input(input);
    let width = *walkable.iter().map(|(_, x)| x).max().unwrap();
    let height = *walkable.iter().map(|(y, _)| y).max().unwrap();
    let out_of_bounds = |pos: &(isize, isize)| pos.0 < 0 || pos.0 > height || pos.1 < 0 || pos.1 > width;

    // Walk
    let mut guard_path = VecDeque::with_capacity(walkable.len());
    loop {
        guard_path.push_back((guard.0, guard.1));
        let next = guard.1.walk(&guard.0);
        if walkable.contains(&next) {
            guard.0 = next;
        } else {
            if out_of_bounds(&next) {
                break;
            } else {
                guard.1 = guard.1.next();
            }
        }
    }

    // Find possible obstacle locations
    let (mut pos, mut dir) = guard_path.pop_front().unwrap();
    let start = pos;
    let mut prev_path = HashSet::with_capacity(walkable.len());
    let mut obstacles = HashSet::with_capacity(walkable.len());
    obstacles.insert(start);
    while let Some((obstacle, ndir)) = guard_path.pop_front() {
        prev_path.insert(pos);
        // Check that the obstacle is not placed in the middle of the previously traced path
        // Also, do not check for an already checked position
        if !prev_path.contains(&obstacle) && !obstacles.contains(&obstacle) {
            let mut visited_loop = Vec::with_capacity(walkable.len());
            visited_loop.push((pos, dir));
            dir = dir.next();
            // Simulate the guard walk with this new obstacle
            loop {
                let next = dir.walk(&pos);
                if out_of_bounds(&next) {
                    // Walked out of the map, this is not a loop
                    break;
                } else if next == obstacle {
                    // Hit the new obstacle: turn right
                    visited_loop.push((pos, dir));
                    dir = dir.next();
                } else if visited_loop.iter().any(|&(v, vd)| (v, vd) == (next, dir)) {
                    // We are repeating a position, closed a loop!
                    obstacles.insert(obstacle);
                    break;
                } else if walkable.contains(&next) {
                    // There is no obstacle
                    pos = next;
                } else {
                    // Hit an already existing obstacle
                    visited_loop.push((pos, dir));
                    dir = dir.next();
                }
            }
        }
        // Update starting position
        (pos, dir) = (obstacle, ndir);
    }
    obstacles.remove(&start);

    obstacles.len()
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 41);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 5153);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 6);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
