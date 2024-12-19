use std::{env,fs,process};
use std::collections::HashMap;

fn count_patterns<'a>(towels: &[&str], patterns: &mut HashMap<&'a str, u64>, pattern: &'a str) -> u64 {
    match patterns.get(pattern) {
        Some(count) => *count,
        None => {
            let count = towels.iter().map(|towel| {
                if let Some(rest) = pattern.strip_suffix(towel) {
                    count_patterns(towels, patterns, rest)
                } else {
                    0
                }
            }).sum();
            patterns.insert(pattern, count);
            count
        },
    }
}

fn run1(input: &str) -> usize {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<_> = first.split(", ").collect();
    let mut patterns = HashMap::from([("", 1)]);
    towels.sort_by(|towel1, towel2| towel1.len().cmp(&towel2.len()));
    (0..towels.len()).for_each(|i| {
        count_patterns(&towels[..=i], &mut patterns, towels[i]);
    });

    second.lines().filter(|line| count_patterns(&towels, &mut patterns, line) > 0).count()
}

fn run2(input: &str) -> u64 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut towels: Vec<_> = first.split(", ").collect();
    let mut patterns = HashMap::from([("", 1)]);
    towels.sort_by(|towel1, towel2| towel1.len().cmp(&towel2.len()));
    (0..towels.len()).for_each(|i| {
        count_patterns(&towels[..=i], &mut patterns, towels[i]);
    });

    second.lines().map(|line| count_patterns(&towels, &mut patterns, line)).sum()
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
    assert_eq!(res, 6);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 296);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 16);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 619970556776002);
}
