use std::{env,fs,process};

fn run(input: &str, part2: bool) -> u32 {
    let mut res = 0;
    let chars_mul = ['m', 'u', 'l', '('];
    let chars_do = ['d', 'o', '(', ')'];
    let chars_dont = ['d', 'o', 'n', '\'', 't', '(', ')'];
    let mut enabled = true;
    let mut index_switch = 0;
    let mut index_mul = 0;
    let mut temp = 0;
    let mut left = None;
    for c in input.chars() {
        if part2 && index_switch < chars_dont.len() && c == chars_dont[index_switch] {
            if c == ')' {
                enabled = false;
                index_switch = 0;
            } else {
                index_switch += 1;
            }
            index_mul = 0;
            left = None;
            temp = 0;
        } else if part2 && index_switch < chars_do.len() && c == chars_do[index_switch] {
            if c == ')' {
                enabled = true;
                index_switch = 0;
            } else {
                index_switch += 1;
            }
            index_mul = 0;
            left = None;
            temp = 0;
        } else if enabled && index_mul < chars_mul.len() && c == chars_mul[index_mul] {
            index_mul += 1;
            index_switch = 0;
        } else if enabled && index_mul == chars_mul.len() {
            if c == ',' && left.is_none() && temp != 0 {
                left = Some(temp);
                temp = 0;
            } else if c.is_digit(10) {
                temp *= 10;
                temp += c.to_digit(10).unwrap();
            } else if c == ')' && left.is_some() && temp != 0 {
                res += left.unwrap() * temp;
                index_mul = 0;
                temp = 0;
                left = None;
            } else {
                // Reset
                index_mul = 0;
                temp = 0;
                left = None;
            }
            index_switch = 0;
        } else {
            // Reset
            index_mul = 0;
            temp = 0;
            left = None;
            index_switch = 0;
        }
    }
    res
}

fn run1(input: &str) -> u32 {
    run(input, false)
}

fn run2(input: &str) -> u32 {
    run(input, true)
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
    assert_eq!(res, 161);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 187833789);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 48);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 94455185);
}
