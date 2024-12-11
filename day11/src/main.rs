use std::{env,fs,process};
use std::collections::HashMap;

fn run(input: &str, blinks: u32) -> usize {
    let stones: Vec<u64> = input.split_whitespace().map(|tok| tok.parse().unwrap()).collect();
    let mut map: HashMap<u64, usize> = stones.iter().fold(HashMap::with_capacity(stones.len()), |mut acc, &s| { *acc.entry(s).or_default() += 1; acc });

    for _ in 0..blinks {
        let mut tmpmap = HashMap::with_capacity((map.len() * 3) / 2);
        for (stone, count) in map {
            if stone == 0 {
                *tmpmap.entry(1).or_default() += count;
            } else {
                let dig = stone.ilog10();
                if dig % 2 == 1 {
                    let fact = 10u64.pow((dig / 2) + 1);
                    *tmpmap.entry(stone / fact).or_default() += count;
                    *tmpmap.entry(stone % fact).or_default() += count;
                } else {
                    *tmpmap.entry(stone * 2024).or_default() += count;
                }
            }
        }
        map = tmpmap;
    }

    map.values().sum()
}

fn run1(input: &str) -> usize {
    run(input, 25)
}

fn run2(input: &str) -> usize {
    run(input, 75)
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
    assert_eq!(res, 55312);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 217443);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 65601038650482);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 257246536026785);
}
