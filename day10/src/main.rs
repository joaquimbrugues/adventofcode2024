use std::{env,fs,process};
use std::collections::HashSet;

fn score(trailhead: &(usize, usize), array: &Vec<Vec<i8>>) -> usize {
    let neighbours = |point: &(usize, usize)| {
        let mut res = Vec::new();
        if point.0 > 0 { res.push((point.0 - 1, point.1)); }
        if point.0 + 1 < array.len() { res.push((point.0 + 1, point.1)); }
        if point.1 > 0 { res.push((point.0, point.1 - 1)); }
        if point.1 + 1 < array[0].len() { res.push((point.0, point.1 + 1)); }
        res
    };

    // DFS
    let mut stack = vec![*trailhead];
    let mut visited = HashSet::new();
    let mut cims = HashSet::new();
    while let Some(p) = stack.pop() {
        if !visited.contains(&p) {
            visited.insert(p);
            if array[p.0][p.1] == 9 {
                cims.insert(p);
            } else {
                for n in neighbours(&p) {
                    if array[n.0][n.1] - array[p.0][p.1] == 1 {
                        stack.push(n);
                    }
                }
            }
        }
    }

    cims.len()
}

fn run1(input: &str) -> usize {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut topo: Vec<Vec<i8>> = Vec::with_capacity(height);
    let mut trailheads = Vec::with_capacity(height * width);
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(width);
        for (j, c) in line.chars().enumerate() {
            if c == '0' {
                trailheads.push((i,j));
            }
            row.push(c.to_digit(10).unwrap() as i8);
        }
        topo.push(row);
    }

    trailheads.into_iter().map(|head| score(&head, &topo)).sum()
}

fn rating(trailhead: &(usize, usize), array: &Vec<Vec<i8>>) -> u32 {
    let neighbours = |point: &(usize, usize)| {
        let mut res = Vec::new();
        if point.0 > 0 { res.push((point.0 - 1, point.1)); }
        if point.0 + 1 < array.len() { res.push((point.0 + 1, point.1)); }
        if point.1 > 0 { res.push((point.0, point.1 - 1)); }
        if point.1 + 1 < array[0].len() { res.push((point.0, point.1 + 1)); }
        res
    };

    // DFS
    let mut stack = vec![(*trailhead, HashSet::new())];
    let mut paths = 0;
    while let Some((p, mut visited)) = stack.pop() {
        if !visited.contains(&p) {
            visited.insert(p);
            if array[p.0][p.1] == 9 {
                paths += 1;
            } else {
                for n in neighbours(&p) {
                    if array[n.0][n.1] - array[p.0][p.1] == 1 {
                        stack.push((n, visited.clone()));
                    }
                }
            }
        }
    }

    paths
}

fn run2(input: &str) -> u32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut topo: Vec<Vec<i8>> = Vec::with_capacity(height);
    let mut trailheads = Vec::with_capacity(height * width);
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(width);
        for (j, c) in line.chars().enumerate() {
            if c == '0' {
                trailheads.push((i,j));
            }
            row.push(c.to_digit(10).unwrap() as i8);
        }
        topo.push(row);
    }

    trailheads.into_iter().map(|head| rating(&head, &topo)).sum()
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
    assert_eq!(res, 36);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 789);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 81);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1735);
}
