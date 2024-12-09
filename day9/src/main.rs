use std::{env,fs,process};
use std::fmt;
use std::collections::VecDeque;

#[derive(Clone,Copy)]
enum Mem {
    Free(usize), // length of contiguous memory
    Occ(usize,usize), // (id, length of contiguous memory)
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Self::Free(len) => {
                let mut deb = String::new();
                for _ in 0..*len { deb.push('.'); }
                write!(f, "{deb}")
            },
            Self::Occ(i, len) => {
                let mut deb = String::new();
                for _ in 0..*len { deb.push_str(format!("{i}").as_str()); }
                write!(f, "{deb}")
            },
        }
    }
}

impl Mem {
    fn free(&self) -> bool {
        match self {
            Self::Free(_) => true,
            _ => false,
        }
    }

    fn inspect(&self) -> usize {
        match self {
            Self::Occ(i,_) => *i,
            Self::Free(_) => panic!("Mem is free!"),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Occ(_, l) => *l,
            Self::Free(l) => *l,
        }
    }
}

fn run1(input: &str) -> usize {
    let mut memory = Vec::with_capacity(input.len());
    for (index, n) in input.trim().chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
        if index % 2 == 0 {
            for _ in 0..n {
                memory.push(Mem::Occ(index / 2, 1));
            }
        } else {
            for _ in 0..n {
                memory.push(Mem::Free(1));
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

fn run2(input: &str) -> usize {
    let mut memory: VecDeque<Mem> = input.trim().chars().enumerate().map(|(index, c)| {
        let n = c.to_digit(10).unwrap() as usize;
        if index % 2 == 0 { Mem::Occ(index / 2, n) }
        else { Mem::Free(n) }
    }).collect();

    let mut defrag: Vec<Mem> = Vec::with_capacity(memory.len());

    while let Some(m) = memory.pop_front() {
        match m {
            Mem::Occ(_,_) => defrag.push(m),
            Mem::Free(mut s) => {
                while let Some((ind, mm)) = memory.iter().enumerate().rev().filter(|(_, mem)| !mem.free() && mem.len() <= s).nth(0) {
                    s -= mm.len();
                    memory.insert(ind+1, Mem::Free(mm.len()));
                    defrag.push(memory.remove(ind).unwrap());
                }
                if s > 0 {
                    defrag.push(Mem::Free(s));
                }
            },
        }
    }
    assert!(defrag.iter().all(|m| m.len() > 0));

    let mut index = 0;
    let mut res = 0;
    for m in defrag {
        let s = m.len();
        for _ in 0..s {
            if !m.free() {
                res += index * m.inspect();
            }
            index += 1;
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
    assert_eq!(res, 1928);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 6366665108136);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2858);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 6398065450842);
}
