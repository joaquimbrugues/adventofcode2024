use std::{env,fs,process};

enum Ops { Sum, Mult, Concat,}

impl Ops {
    fn ev(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Sum => left + right,
            Self::Mult => left * right,
            Self::Concat => format!("{left}{right}").parse().unwrap(),
        }
    }

    const PART1: [Self; 2] = [ Self::Sum, Self::Mult ];
    const PART2: [Self; 3] = [ Self::Sum, Self::Mult, Self::Concat ];
}

fn run1(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let target: u64 = left.parse().unwrap();
        let mut nums = right.split_whitespace().map(|token| token.parse().unwrap());
        let mut stack: Vec<u64> = vec![nums.next().unwrap()];
        for n in nums {
            let mut new_stack = Vec::with_capacity(Ops::PART1.len() * stack.len());
            for acc in stack.iter() {
                for op in Ops::PART1 {
                    let r = op.ev(*acc, n);
                    if r > target { continue; }
                    else { new_stack.push(r); }
                }
            }
            stack = new_stack;
        }
        if stack.into_iter().any(|n| n == target) { res += target; }
    }
    res
}

fn run2(input: &str) -> u64 {
    let mut res = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let target: u64 = left.parse().unwrap();
        let mut nums = right.split_whitespace().map(|token| token.parse().unwrap());
        let mut stack: Vec<u64> = vec![nums.next().unwrap()];
        for n in nums {
            let mut new_stack = Vec::with_capacity(Ops::PART2.len() * stack.len());
            for acc in stack.iter() {
                for op in Ops::PART2 {
                    let r = op.ev(*acc, n);
                    if r > target { continue; }
                    else { new_stack.push(r); }
                }
            }
            stack = new_stack;
        }
        if stack.into_iter().any(|n| n == target) { res += target; }
    }
    res
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
    assert_eq!(res, 3749);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 5837374519342);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 11387);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 492383931650959);
}
