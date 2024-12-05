use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> u32 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(u8, u8)> = first.lines().map(|line| {
        let (s1, s2) = line.split_once('|').unwrap();
        (s1.parse().unwrap(), s2.parse().unwrap())
    }).collect();

    let mut res = 0;
    for line in second.lines() {
        let nums: Vec<u8> = line.split(',').map(|tok| tok.parse().unwrap()).collect();
        let mut seen = HashSet::new();

        for f in nums.iter() {
            let mut compliant = true;
            for (_, s) in rules.iter().filter(|(a,_)| a == f) {
                if seen.contains(s) {
                    compliant = false;
                    break;
                }
            }
            if compliant {
                seen.insert(f);
            } else {
                break;
            }
        }

        let n = nums.len();

        if n == seen.len() {
            res += nums[n / 2] as u32;
        }
    }
    res
}

fn run2(input: &str) -> u32 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules: HashSet<(u8, u8)> = first.lines().map(|line| {
        let (s1, s2) = line.split_once('|').unwrap();
        (s1.parse().unwrap(), s2.parse().unwrap())
    }).collect();

    let mut res = 0;
    for line in second.lines() {
        let mut nums = vec![];
        let mut corrected = false;
        for num in line.split(',').map(|tok| tok.parse().unwrap()) {
            let mut i = 0;
            while i < nums.len() && !rules.contains(&(num, nums[i])) { i += 1; }
            if i < nums.len() {
                corrected = true;
            }
            nums.insert(i,num);
        }
        if corrected {
            res += nums[nums.len() / 2] as u32;
        }
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
    assert_eq!(res, 143);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 6498);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 123);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 5017);
}
