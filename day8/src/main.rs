use std::{env,fs,process};
use std::collections::{HashMap,HashSet};

fn antinodes1(x: &(isize, isize), y: &(isize, isize)) -> [(isize, isize); 2] {
    [
        (2 * x.0 - y.0, 2 * x.1 - y.1),
        (2 * y.0 - x.0, 2 * y.1 - x.1),
    ]
}

fn run1(input: &str) -> usize {
    let antennas = {
        let mut map = HashMap::with_capacity(2500);
        for (y, line) in input.lines().enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                if c != '.' {
                    map.entry(c).or_insert(Vec::new()).push((y,x));
                }
            }
        }
        map
    };
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().chars().count() as isize;
    let inside_bounds = move |point: &(isize, isize)| point.0 >= 0 && point.1 >= 0 && point.0 < height && point.1 < width;

    let n = antennas.len();
    let mut answers = HashSet::with_capacity(n * (n - 1));

    for positions in antennas.values() {
        for i in 0..(positions.len() - 1) {
            for j in (i+1)..positions.len() {
                for a in antinodes1(&positions[i], &positions[j]) {
                    if inside_bounds(&a) {
                        answers.insert(a);
                    }
                }
            }
        }
    }
    
    answers.len()
}

fn antinodes2(x: &(isize, isize), y: &(isize, isize), height: isize, width: isize) -> HashSet<(isize, isize)> {
    let mut res = HashSet::new();
    let inside_bounds = move |point: &(isize, isize)| point.0 >= 0 && point.1 >= 0 && point.0 < height && point.1 < width;
    let v = (y.0 - x.0, y.1 - x.1);
    let mut p = *x;
    while inside_bounds(&p) {
        res.insert(p);
        p = (p.0 + v.0, p.1 + v.1);
    }
    p = (x.0 - v.0, x.1 - v.1);
    while inside_bounds(&p) {
        res.insert(p);
        p = (p.0 - v.0, p.1 - v.1);
    }

    res
}

fn run2(input: &str) -> usize {
    let antennas = {
        let mut map = HashMap::with_capacity(2500);
        for (y, line) in input.lines().enumerate() {
            let y = y as isize;
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                if c != '.' {
                    map.entry(c).or_insert(Vec::new()).push((y,x));
                }
            }
        }
        map
    };
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().chars().count() as isize;

    let n = antennas.len();
    let mut answers = HashSet::with_capacity(n * (n - 1));

    for positions in antennas.values() {
        for i in 0..(positions.len() - 1) {
            for j in (i+1)..positions.len() {
                answers = answers.union(&antinodes2(&positions[i], &positions[j], height, width)).copied().collect();
            }
        }
    }
    
    answers.len()
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
    assert_eq!(res, 14);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 323);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 34);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
