use std::{env,fs,process};

fn check_nums(nums: &[i32]) -> bool {
    let mut sign = 0;
    for d in nums.iter().zip(nums.iter().skip(1)).map(|(n1, n2)| n1 - n2) {
        let a = d.abs();
        if a < 1 || a > 3 { return false; }
        match sign {
            0 => sign = if d > 0 { 1 } else { -1 },
            1 => if d <= 0 { return false; },
            -1 => if d >= 0 { return false; },
            _ => unreachable!(),
        }
    }
    true
}

fn run1(input: &str) -> usize {
    input.lines().filter(|line| {
        let nums: Vec<i32> = line.split_whitespace().map(|tok| tok.parse().unwrap()).collect();
        check_nums(&nums)
    }).count()
}

fn run2(input: &str) -> usize {
    input.lines().filter(|line| {
        let nums: Vec<i32> = line.split_whitespace().map(|tok| tok.parse().unwrap()).collect();
        if check_nums(&nums) {
            return true;
        } else {
            for i in 0..nums.len() {
                let mut nn = nums.clone();
                nn.remove(i);
                if check_nums(&nn) { return true; }
            }
        }
        false
    }).count()
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
    assert_eq!(res, 2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 236);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 4);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 308);
}
