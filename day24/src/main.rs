use std::{env,fs,process,fmt};
use std::collections::HashMap;

enum Op { Xor, Or, And, }

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Xor => "XOR",
            Self::Or => "OR",
            Self::And => "AND",
        };
        write!(f, "{s}")
    }
}

impl From<&str> for Op {
    fn from(label: &str) -> Self {
        match label {
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            "AND" => Self::And,
            _ => panic!("Unexpected label {label}"),
        }
    }
}

impl Op {
    fn ev(&self, left: bool, right: bool) -> bool {
        match self {
            Self::Or => left || right,
            Self::And => left && right,
            Self::Xor => left ^ right,
        }
    }
}

fn compute_wire_value<'a>(label: &str, wire_values: &mut HashMap<&'a str, bool>, gates: &'a HashMap<&'a str, (&str, Op, &str)>) -> bool {
    if let Some(v) = wire_values.get(label) {
        *v
    } else {
        // Cast label to avoid having to link the lifetime of wire_values with that of label (which
        // might be constructed from String later on and dropped prematurely)
        let (label, (lleft, op, lright)) = gates.get_key_value(label).unwrap();
        let b1 = compute_wire_value(lleft, wire_values, gates);
        let b2 = compute_wire_value(lright, wire_values, gates);
        let v = op.ev(b1, b2);
        wire_values.insert(label, v);
        v
    }
}

fn run1(input: &str) -> u64 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut wire_values: HashMap<&str, bool> = first.lines().map(|line| {
        let (label, sv) = line.split_once(": ").unwrap();
        let v = match sv {
            "0" => false,
            "1" => true,
            _ => panic!("Unexpected value {sv}"),
        };
        (label, v)
    }).collect();

    let gates: HashMap<&str, (&str, Op, &str)> = second.lines().map(|line| {
        let (rest, label) = line.split_once(" -> ").unwrap();
        let mut tokens = rest.split_whitespace();
        (label, (tokens.next().unwrap(), tokens.next().unwrap().into(), tokens.next().unwrap()))
    }).collect();

    let mut counter: isize = 45;   // 45 is the largest label in the input
    let mut res = 0;

    while counter >= 0 {
        let label = format!("z{:0>2}", counter);
        if gates.contains_key(label.as_str()) {
            let b = compute_wire_value(label.as_str(), &mut wire_values, &gates);
            res <<= 1;
            res += b as u64;
        }
        counter -= 1;
    }

    res
}

fn run2(input: &str) -> u32 {
    0
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

    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1p1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 0b100);
}

#[test]
fn example2p1() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 0b0011111101000);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 52038112429798);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
