use std::{env,fs,process};

fn run1(input: &str) -> i32 {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut toks = line.split_whitespace();
        let n1: i32 = toks.next().unwrap().parse().unwrap();
        let mut i = 0;
        while i < list1.len() && list1[i] < n1 {
            i += 1;
        }
        list1.insert(i, n1);
        i = 0;
        let n2: i32 = toks.next().unwrap().parse().unwrap();
        while i < list2.len() && list2[i] < n2 {
            i += 1;
        }
        list2.insert(i, n2);
    }
    list1.into_iter().zip(list2.into_iter()).map(|(n1, n2)| (n1 - n2).abs()).sum()
}

fn run2(input: &str) -> u32 {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut toks = line.split_whitespace();
        let n1: u32 = toks.next().unwrap().parse().unwrap();
        let mut i = 0;
        while i < list1.len() && list1[i] < n1 {
            i += 1;
        }
        list1.insert(i, n1);
        i = 0;
        let n2: u32 = toks.next().unwrap().parse().unwrap();
        while i < list2.len() && list2[i] < n2 {
            i += 1;
        }
        list2.insert(i, n2);
    }

    let mut sum = 0;
    for n1 in list1 {
        let mut i = 0;
        while i < list2.len() && list2[i] < n1 {
            i += 1;
        }
        let mut c = 0;
        while i < list2.len() && list2[i] == n1 {
            i += 1;
            c += 1;
        }
        sum += n1 * c;
    }
    sum
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
    assert_eq!(res, 11);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 3574690);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 31);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 22565391);
}
