use std::{env,fs,process};
use std::collections::{BinaryHeap,HashSet};
use std::cmp::Ordering;

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
enum Dir { N, E, S, W, }

impl Dir {
    fn turn(&self) -> [Self; 2] {
        match self {
            Self::N => [Self::W, Self::E],
            Self::S => [Self::W, Self::E],
            Self::W => [Self::N, Self::S],
            Self::E => [Self::N, Self::S],
        }
    }

    fn walk(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::N => (pos.0 - 1, pos.1),
            Self::S => (pos.0 + 1, pos.1),
            Self::W => (pos.0, pos.1 - 1),
            Self::E => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
struct Pos { coord: (usize, usize), ori: Dir, score: u32, }

impl Pos {
    fn walk(&mut self) {
        self.coord = self.ori.walk(self.coord);
        self.score += 1;
    }

    fn turn(&self) -> [Self; 2] {
        let oris = self.ori.turn();
        [
            Self { coord: self.coord, ori: oris[0], score: self.score + 1000 },
            Self { coord: self.coord, ori: oris[1], score: self.score + 1000 },
        ]
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, (usize, usize), (usize, usize)) {
    let mut start = (0,0);
    let mut end = (0,0);
    let mut walkable = HashSet::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => {},
                '.' => { walkable.insert((y,x)); },
                'S' => {
                    start = (y,x);
                    walkable.insert((y,x));
                },
                'E' => {
                    end = (y,x);
                    walkable.insert((y,x));
                },
                _ => unreachable!(),
            }
        });
    });

    (walkable, start, end)
}

fn run1(input: &str) -> u32 {
    let (walkable, start, end) = parse_input(input);

    let ini = Pos { coord: start, ori: Dir::E, score: 0 };
    let mut pqueue = BinaryHeap::from([ini]);
    let mut visited = HashSet::from([ini]);

    while let Some(mut pos) = pqueue.pop() {
        if pos.coord == end {
            return pos.score;
        } else {
            let [left, right] = pos.turn();
            pos.walk();
            for p in [pos, left, right] {
                if walkable.contains(&p.coord) && !visited.contains(&p) {
                    visited.insert(p);
                    pqueue.push(p);
                }
            }
        }
    }
    unreachable!()
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
fn example1p1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 7036);
}

#[test]
fn example2p1() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 11048);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 78428);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
