use std::{env,fs,process};
use std::fmt;

enum Mem {
    Free,
    Occ(usize),
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Self::Free => write!(f, "."),
            Self::Occ(i) => write!(f, "{i}"),
        }
    }
}

impl Mem {
    fn free(&self) -> bool {
        match self {
            Self::Free => true,
            _ => false,
        }
    }

    fn inspect(&self) -> usize {
        match self {
            Self::Occ(i) => *i,
            Self::Free => panic!("Mem is free!"),
        }
    }
}

fn run1(input: &str) -> usize {
    let mut memory = Vec::with_capacity(input.len());
    for (index, n) in input.trim().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        if index % 2 == 0 {
            for _ in 0..n {
                memory.push(Mem::Occ(index / 2));
            }
        } else {
            for _ in 0..n {
                memory.push(Mem::Free);
            }
        }
    }

    let mut in0 = 0;
    let mut in1 = memory.len() - 1;
    while in0 < in1 {
        if memory[in0].free() {
            if memory[in1].free() {
                in1 -= 1;
            } else {
                memory.swap(in0, in1);
            }
        } else {
            in0 += 1;
        }
    }

    memory.into_iter().filter(|m| !m.free()).enumerate().map(|(i,m)| i * m.inspect()).sum()
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
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1928);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 6366665108136);
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
