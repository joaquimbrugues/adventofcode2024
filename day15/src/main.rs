use std::{env,fs,process};
use std::collections::{HashSet, VecDeque};

enum Dir { N, E, S, W, }

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            _ => panic!("Unexpected character {c}"),
        }
    }
}

impl Dir {
    fn walk(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::N => (pos.0 - 1, pos.1),
            Self::S => (pos.0 + 1, pos.1),
            Self::W => (pos.0, pos.1 - 1),
            Self::E => (pos.0, pos.1 + 1),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::N => '^',
            Self::S => 'v',
            Self::W => '<',
            Self::E => '>',
        }
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Cont { Free, Crate, Wall, }

impl From<char> for Cont {
    fn from(c: char) -> Self {
        match c {
            '@' | '.' => Self::Free,
            '#' => Self::Wall,
            'O' => Self::Crate,
            _ => panic!("Unexpected character {c}"),
        }
    }
}

fn run1(input: &str) -> usize {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<Cont>> = Vec::new();
    let mut robot = (0,0);
    for (y, line) in first.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                robot = (y,x);
            }
            row.push(c.into());
        }
        map.push(row);
    }

    for d in second.lines().map(|line| line.chars()).flatten().map(|c| Dir::from(c)) {
        let mut next = d.walk(robot);
        match map[next.0][next.1] {
            Cont::Wall => {},
            Cont::Free => robot = next,
            Cont::Crate => {
                let from = next;
                while map[next.0][next.1] == Cont::Crate { next = d.walk(next); }
                if map[next.0][next.1] == Cont::Free {
                    map[from.0][from.1] = Cont::Free;
                    robot = from;
                    map[next.0][next.1] = Cont::Crate;
                }
            },
        }
    }

    map.into_iter().enumerate().map(|(y, row)| row.into_iter().enumerate().map(|(x, c)| {
        match c {
            Cont::Crate => 100 * y + x,
            _ => 0,
        }
    }).sum::<usize>()).sum()
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum CrateDir {L, R,}

impl CrateDir {
    fn switch(&self) -> Self {
        match self {
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum WCont { Free, Wall, Crate(CrateDir), }

impl WCont {
    fn from_char(c: char) -> (Self, Self) {
        match c {
            '@' | '.' => (Self::Free, Self::Free),
            '#' => (Self::Wall, Self::Wall),
            'O' => (Self::Crate(CrateDir::L), Self::Crate(CrateDir::R)),
            _ => panic!("Unexpected char {c}"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Free => '.',
            Self::Wall => '#',
            Self::Crate(CrateDir::L) => '[',
            Self::Crate(CrateDir::R) => ']',
        }
    }
}

fn print_map(map: &Vec<Vec<WCont>>, robot: (usize, usize)) {
    for (y, row) in map.iter().enumerate() {
        print!("{y:0>3} ");
        for (x, c) in row.iter().enumerate() {
            if robot == (y,x) {
                assert_eq!(*c, WCont::Free);
                print!("@");
            } else {
                print!("{}", c.to_char());
            }
        }
        print!(" {y:0>3}\n");
    }
    print!("    ");
    for i in 0..map[0].len() {
        print!("{}", i / 10);
    }
    print!("    \n");
    print!("    ");
    for i in 0..map[0].len() {
        print!("{}", i % 10);
    }
    print!("    \n");
}

fn run2(input: &str) -> usize {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<WCont>> = Vec::new();
    let mut robot = (0,0);
    for (y, line) in first.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '@' { robot = (y,2*x); }
            let (l,r) = WCont::from_char(c);
            row.push(l);
            row.push(r);
        }
        map.push(row);
    }

    //println!("Initial state");
    //print_map(&map, robot);

    for d in second.lines().map(|line| line.chars()).flatten().map(|c| Dir::from(c)) {
        //println!("\nMove {}:", d.to_char());
        let next = d.walk(robot);
        match map[next.0][next.1] {
            WCont::Free => robot = next,
            WCont::Wall => {},
            WCont::Crate(dir) => {
                let mut to_check = VecDeque::from([(next, dir)]);
                let mut to_move = Vec::new();
                let mut checked = HashSet::new();
                while let Some((pos, rel_pos)) = to_check.pop_front() {
                    if checked.contains(&pos) {
                        continue;
                    } else {
                        checked.insert(pos);
                    }
                    let other = match rel_pos {
                        CrateDir::L => (pos.0, pos.1+1),
                        CrateDir::R => (pos.0, pos.1-1),
                    };
                    to_check.push_back((other, rel_pos.switch()));

                    let next = d.walk(pos);
                    match map[next.0][next.1] {
                        WCont::Free => to_move.push((pos, next)),
                        WCont::Wall => {
                            to_move = vec![];
                            break;
                        },
                        WCont::Crate(nd) => {
                            to_move.push((pos, next));
                            to_check.push_back((next, nd));
                        },
                    }
                }
                if !to_move.is_empty() {
                    //println!("Gonna move {} crates!", to_move.len());
                    //println!("{to_move:?}");
                    while let Some((old, new)) = to_move.pop() {
                        assert_eq!(map[new.0][new.1], WCont::Free);
                        map[new.0][new.1] = map[old.0][old.1];
                        map[old.0][old.1] = WCont::Free;
                    }
                    robot = next;
                }
            },
        }
        //print_map(&map, robot);
    }

    map.into_iter().enumerate().map(|(y,row)| row.into_iter().enumerate().map(|(x,c)| {
        match c {
            WCont::Crate(CrateDir::L) => 100 * y + x,
            _ => 0,
        }
    }).sum::<usize>()).sum()
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
    assert_eq!(res, 10092);
}

#[test]
fn example12() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2028);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1517819);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 9021);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1538862);
}
