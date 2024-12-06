use std::{env,fs,process};
use std::collections::HashSet;

#[derive(Debug)]
enum Dir { N, S, W, E, }

impl Dir {
    fn next(self) -> Self {
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

fn run1(input: &str) -> usize {
    // Read input
    let mut guard = None;
    let mut walkable = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            match c {
                '#' => {},
                '.' => { walkable.insert((y,x)); },
                '^' => {
                    guard.insert(((y,x), Dir::N));
                    walkable.insert((y,x));
                },
                _ => unreachable!(),
            }
        }
    }
    let mut guard = guard.unwrap();
    let width = *walkable.iter().map(|(_, x)| x).max().unwrap();
    let height = *walkable.iter().map(|(y, _)| y).max().unwrap();

    // Walk
    let mut visited = HashSet::with_capacity(walkable.len());
    loop {
        visited.insert(guard.0);
        let next = guard.1.walk(&guard.0);
        if walkable.contains(&next) {
            guard.0 = next;
        } else {
            if next.0 < 0 || next.0 > height || next.1 < 0 || next.1 > width {
                break;
            } else {
                guard.1 = guard.1.next();
            }
        }
    }

    visited.len()
}

fn run2(input: &str) -> u32 {
    0
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

    let res = run1(&input);
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
