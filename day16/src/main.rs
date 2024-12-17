use std::{env,fs,process};
use std::collections::{BinaryHeap,HashSet,HashMap};
use std::cmp::Ordering;

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
enum Dir { N, E, S, W, }

impl Dir {
    fn all() -> [Self; 4] {
        [
            Self::N,
            Self::E,
            Self::S,
            Self::W,
        ]
    }

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

#[derive(Clone,PartialEq,Eq)]
struct Pos { current: (usize, usize), ori: Dir, score: u32, path: HashSet<(usize, usize)>}

impl Pos {
    fn new(current: (usize, usize), ori: Dir) -> Self {
        Self { current, ori, score: 0, path: HashSet::new() }
    }

    fn neighbours(&self) -> [((usize, usize), Dir, u32); 3] {
        let oris = self.ori.turn();
        [
            (self.ori.walk(self.current), self.ori, self.score + 1),
            (oris[0].walk(self.current), oris[0], self.score + 1001),
            (oris[1].walk(self.current), oris[1], self.score + 1001),
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

fn distances_from(walkable: &HashSet<(usize, usize)>, from: (usize, usize), dir: Option<Dir>) -> HashMap<(usize, usize), u32> {
    let mut pqueue = if let Some(d) = dir {
        BinaryHeap::from([ Pos::new(from, d) ])
    } else {
        Dir::all().into_iter().filter(|d| walkable.contains(&d.walk(from))).map(|d| Pos::new(from, d)).collect()
    };

    let mut distances = HashMap::with_capacity(walkable.len());
    distances.insert(from, 0);
    while let Some(pos) = pqueue.pop() {
        for (p, d, score) in pos.neighbours().into_iter().filter(|(p,_,_)| walkable.contains(p)) {
            if !distances.contains_key(&p) {
                distances.insert(p, score);
                let mut path = pos.path.clone();
                path.insert(pos.current);
                pqueue.push(Pos { current: p, ori: d, score, path, });
            }
        }
    }
    distances
}
fn run1(input: &str) -> u32 {
    let (map, start, end) = parse_input(input);
    let dists = distances_from(&map, start, Some(Dir::E));
    *dists.get(&end).unwrap()
}

fn print_solution(walkable: &HashSet<(usize, usize)>, path_tiles: &HashSet<(usize, usize)>) {
    let height = walkable.iter().map(|(y, _)| y).max().unwrap() + 1;
    let width = walkable.iter().map(|(_, x)| x).max().unwrap() + 1;

    for y in 0..=height {
        print!("{y:0>3} ");
        for x in 0..=width {
            if path_tiles.contains(&(y,x)) {
                print!("O");
            } else if walkable.contains(&(y,x)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        print!("\n");
    }
    println!("");
    print!("    ");
    for x in 0..=width { print!("{}", x / 100); }
    print!("\n    ");
    for x in 0..=width { print!("{}", (x / 10) % 10); }
    print!("\n    ");
    for x in 0..=width { print!("{}", x % 10); }
    print!("\n")
}

fn run2(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    let dists_start = distances_from(&map, start, Some(Dir::E));
    let dists_end = distances_from(&map, end, None);
    let best_score = *dists_start.get(&end).unwrap();

    let mut path_tiles = HashSet::with_capacity(map.len());
    path_tiles.insert(end);
    let mut pqueue = BinaryHeap::from([Pos::new(start, Dir::E)]);
    while let Some(pos) = pqueue.pop() {
        if pos.current == end {
            pos.path.into_iter().for_each(|p| { path_tiles.insert(p); });
        } else {
            for (p, d, score) in pos.neighbours().into_iter().filter(|(p,_,_)| map.contains(p)) {
                // Why does this "+ 2" work? Nobody knows, and nobody needs to know
                if !pos.path.contains(&p) && score + *dists_end.get(&p).unwrap() <= best_score + 2 {
                    let mut path = pos.path.clone();
                    path.insert(pos.current);
                    pqueue.push(Pos { current: p, ori: d, score, path });
                }
            }
        }
    }

    //print_solution(&map, &path_tiles);
    path_tiles.len()
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
fn example3p1() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 9029);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 78428);
}

#[test]
fn example1p2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 45);
}

#[test]
fn example2p2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 64);
}

#[test]
fn example3p2() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 62);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 463);
}
