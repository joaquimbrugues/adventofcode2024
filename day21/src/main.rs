use std::{env,fs,process,fmt,iter};
use std::collections::{HashMap,HashSet};

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
enum Dir {U, D, L, R,}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let c = match self {
            Self::U => '^',
            Self::D => 'v',
            Self::L => '<',
            Self::R => '>',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Key { N(u8), D(Dir), A, }

impl From<char> for Key {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Self::N(c.to_digit(10).unwrap() as u8),
            'A' => Self::A,
            _ => panic!("Unexpected character {c}"),
        }
    }
}

impl From<Dir> for Key {
    fn from(d: Dir) -> Self {
        Self::D(d)
    }
}

const NUM_KEYPAD: [(Key, (isize, isize)); 11] = [
    (Key::N(7), (0,0)), (Key::N(8), (0,1)), (Key::N(9), (0,2)),
    (Key::N(4), (1,0)), (Key::N(5), (1,1)), (Key::N(6), (1,2)),
    (Key::N(1), (2,0)), (Key::N(2), (2,1)), (Key::N(3), (2,2)),
    (Key::N(0), (3,1)), (Key::A, (3,2)),
];

const DIR_KEYPAD: [(Key, (isize, isize)); 5] = [
    (Key::D(Dir::U), (0,1)), (Key::A, (0,2)),
    (Key::D(Dir::L), (1,0)), (Key::D(Dir::D), (1,1)), (Key::D(Dir::R), (1,2)),
];

struct Keypad { trans: HashMap<Key, (isize, isize)>, keys: HashSet<(isize, isize)>, }

impl From<HashMap<Key, (isize, isize)>> for Keypad {
    fn from(trans: HashMap<Key, (isize, isize)>) -> Self {
        let keys = trans.values().copied().collect();
        Self { trans, keys, }
    }
}

fn get_vertical(n: isize) -> Vec<Dir> {
    match n.signum() {
        -1 => vec![Dir::U; n.abs() as usize],
        1 => vec![Dir::D; n.abs() as usize],
        0 => vec![],
        _ => unreachable!(),
    }
}

fn get_horizontal(n: isize) -> Vec<Dir> {
    match n.signum() {
        -1 => vec![Dir::L; n.abs() as usize],
        1 => vec![Dir::R; n.abs() as usize],
        0 => vec![],
        _ => unreachable!(),
    }
}

impl Keypad {
    fn movement(&self, from: &Key, to: &Key) -> Vec<Dir> {
        let from = *self.trans.get(from).unwrap();
        let to = *self.trans.get(to).unwrap();
        match (to.0 - from.0, to.1 - from.1) {
            (0,0) => vec![],
            (0, x) => { // horizontal
                get_horizontal(x)
            },
            (y, 0) => {  // vertical
                get_vertical(y)
            },
            (y,x) => match x.signum() {   // diagonal
                -1 => {
                    if self.keys.contains(&(from.0, to.1)) {
                        let mut res = get_horizontal(x);
                        res.append(&mut get_vertical(y));
                        res
                    } else {
                        let mut res = get_vertical(y);
                        res.append(&mut get_horizontal(x));
                        res
                    }
                },
                1 => {
                    if self.keys.contains(&(to.0, from.1)) {
                        let mut res = get_vertical(y);
                        res.append(&mut get_horizontal(x));
                        res
                    } else {
                        let mut res = get_horizontal(x);
                        res.append(&mut get_vertical(y));
                        res
                    }
                },
                _ => unreachable!(),
            },
        }
    }
}

fn translate(keystrokes: &Vec<Key>, keypad: &Keypad) -> Vec<Key> {
    let mut prev = Key::A;
    let mut res = Vec::new();
    for k in keystrokes {
        keypad.movement(&prev, k).into_iter().for_each(|d| res.push(d.into()));
        res.push(Key::A);
        prev = *k;
    }
    res
}

fn recursive_compute(memoized_costs: &mut HashMap<(Key, Key, u8), u64>, dir_keypad: &Keypad, from: Key, to: Key, height: u8) -> u64 {
    if let Some(count) = memoized_costs.get(&(from, to, height)) {
        return *count;
    } else {
        let count = if height == 0 || from == to {
            // Base case
            1
        } else {
            //Recursion
            let next_seq = dir_keypad.movement(&from, &to);
            iter::once(Key::A).chain(next_seq.iter().map(|&d| d.into())).zip(next_seq.iter().map(|&d| d.into()).chain(iter::once(Key::A))).map(|(k1, k2)| {
                recursive_compute(memoized_costs, dir_keypad, k1, k2, height - 1)
            }).sum()
        };
        memoized_costs.insert((from, to, height), count);
        count
    }
}

fn run(input: &str, num_of_dir_robots: u8) -> u64 {
    let num_keypad = Keypad::from(HashMap::from(NUM_KEYPAD));
    let dir_keypad = Keypad::from(HashMap::from(DIR_KEYPAD));
    let mut memoized_costs: HashMap<(Key, Key, u8), u64> = HashMap::with_capacity(625);
    input.lines().map(|code| {
        let keystrokes = code.chars().map(|c| c.into()).collect();
        let seq = translate(&keystrokes, &num_keypad);
        let len: u64 = iter::once(&Key::A).chain(seq.iter()).zip(seq.iter()).map(|(&k1, &k2)| recursive_compute(&mut memoized_costs, &dir_keypad, k1, k2, num_of_dir_robots)).sum();
        let code: u64 = code.strip_suffix('A').unwrap().parse().unwrap();
        code * len
    }).sum()
}

fn run1(input: &str) -> u64 {
    run(input, 2)
}

fn run2(input: &str) -> u64 {
    run(input, 25)
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
    assert_eq!(res, 126384);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 174124);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 154115708116294);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 216668579770346);
}
