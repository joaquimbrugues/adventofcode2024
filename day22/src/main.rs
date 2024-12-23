use std::{env,fs,process};
use std::collections::{HashMap,HashSet};

const PRUNE: u64 = 0b1_000_000_000_000_000_000_000_000;

fn generate(n: u64) -> u64 {
    let mut res = n ^ (n << 6); // Mix with the result of multiplying by 64
    res %= PRUNE; // Prune via modulo
    res ^= res >> 5;    // Mix with the result of dividing by 2^5
    res %= PRUNE; // Prune
    res ^= res << 11;  // Mix with the result of multiplying by 2024
    res %= PRUNE; // Prune
    
    res
}

fn run1(input: &str) -> u64 {
    input.lines().map(|line| {
        let mut n = line.parse().unwrap();
        for _ in 0..2000 {
            n = generate(n);
        }
        n
    }).sum()
}

fn run2(input: &str) -> u16 {
    let mut cached = HashMap::with_capacity(130_321);
    for line in input.lines() {
        let mut seen = HashSet::new();
        let mut prev = line.parse().unwrap();
        let mut prices = Vec::with_capacity(2_000);
        for _ in 0..2_000 {
            let next = generate(prev);
            prices.push(((next % 10) as u16, ((next % 10) as i8) - ((prev % 10) as i8)));
            prev = next;
        }

        for i in 3..2_000 {
            if seen.insert((prices[i-3].1, prices[i-2].1, prices[i-1].1, prices[i].1)) {
                *cached.entry((prices[i-3].1, prices[i-2].1, prices[i-1].1, prices[i].1)).or_insert(0) += prices[i].0;
            }
        }
    }
    *cached.values().max().unwrap()
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
    assert_eq!(res, 37327623);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 21147129593);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 23);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2445);
}
