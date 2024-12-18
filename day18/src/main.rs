use std::{env,fs,process};
use std::collections::{VecDeque,HashSet};

fn make_map(input: &str, fallen: usize) -> HashSet<(isize, isize)> {
    input.lines().take(fallen).map(|line| {
        let (t1, t2) = line.split_once(',').unwrap();
        (t1.parse().unwrap(), t2.parse().unwrap())
    }).collect()
}

fn neighbours(point: (isize, isize), grid_size: isize) -> Vec<(isize, isize)> {
    let abs_neighs = [
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ];
    abs_neighs.into_iter().filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < grid_size && p.1 < grid_size).collect()
}

fn bfs(obstacles: &HashSet<(isize, isize)>, grid_size: isize, start: (isize, isize), target: (isize, isize)) -> Option<usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);

    while let Some((point, dist)) = queue.pop_front() {
        if point == target {
            return Some(dist);
        } else {
            for n in neighbours(point, grid_size) {
                if !(obstacles.contains(&n) || visited.contains(&n)) {
                    visited.insert(n);
                    queue.push_back((n, dist + 1));
                }
            }
        }
    }
    None
}

fn run1(input: &str, grid_size: isize, fallen: usize) -> usize {
    let obstacles = make_map(input, fallen);
    bfs(&obstacles, grid_size, (0,0), (grid_size - 1, grid_size - 1)).unwrap()
}

fn run2(input: &str, grid_size: isize, skip: usize) -> (isize, isize) {
    let mut obstacles = make_map(input, skip);
    for bit in input.lines().skip(skip).map(|line| { let (t1, t2) = line.split_once(',').unwrap(); (t1.parse().unwrap(), t2.parse().unwrap()) }) {
        obstacles.insert(bit);
        if bfs(&obstacles, grid_size, (0,0), (grid_size - 1, grid_size - 1)).is_none() {
            return bit;
        }
    }
    unreachable!();
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

    let res = run2(&input, 71, 1024);
    println!("{res:?}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 7, 12);
    assert_eq!(res, 22);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input, 71, 1024);
    assert_eq!(res, 290);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input, 7, 12);
    assert_eq!(res, (6,1));
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input, 71, 1024);
    assert_eq!(res, (64,54));
}
