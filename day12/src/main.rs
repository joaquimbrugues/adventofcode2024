use std::{env,fs,process};
use std::collections::{HashMap,HashSet,VecDeque};

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
enum Dir { N, E, S, W, }

impl Dir {
    fn all() -> [Self; 4] {
        [
            Self::N,
            Self::E,
            Self::S,
            Self::W,
        ]
    }

    fn op(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::W => Self::E,
            Self::E => Self::W,
        }
    }
}

fn neighbours((i,j): (isize, isize)) -> [((isize, isize), Dir); 4] {
    [
        ((i-1, j), Dir::N),
        ((i+1, j), Dir::S),
        ((i, j-1), Dir::W),
        ((i, j+1), Dir::E),
    ]
}

fn run1(input: &str) -> u32 {
    let map: HashMap<(isize, isize), char> = input.lines().enumerate().map(|(i, line)| line.chars().enumerate().map(|(j,c)| ((i as isize,j as isize), c)).collect::<Vec<_>>()).flatten().collect();

    let mut res = 0;
    let mut checked = HashSet::with_capacity(map.len());

    for (p, c) in &map {
        if checked.contains(p) { continue; }
        let mut visited = HashSet::with_capacity(map.len());
        let mut queue = VecDeque::from([*p]);
        visited.insert(*p);
        let mut perimeter = 0;

        while let Some(p) = queue.pop_front() {
            for (n, _) in neighbours(p) {
                if !visited.contains(&n) {
                    if map.get(&n).is_some_and(|d| c == d) {
                        visited.insert(n);
                        queue.push_back(n);
                    } else {
                        perimeter += 1;
                    }
                }
            }
        }
        let area = visited.len() as u32;
        res += perimeter * area;
        checked = checked.union(&visited).copied().collect();
    }

    res
}

fn diagonals(p: (isize, isize)) -> [(isize, isize); 4] {
    [
        (p.0 - 1, p.1 - 1),
        (p.0 - 1, p.1 + 1),
        (p.0 + 1, p.1 - 1),
        (p.0 + 1, p.1 + 1),
    ]
}

fn diagonals_with_dir(p: (isize, isize), dir: Dir) -> [(isize, isize); 2] {
    match dir {
        Dir::N => [(p.0-1, p.1-1), (p.0-1,p.1+1)],
        Dir::S => [(p.0+1, p.1-1), (p.0+1,p.1+1)],
        Dir::W => [(p.0-1, p.1-1), (p.0+1,p.1-1)],
        Dir::E => [(p.0-1, p.1+1), (p.0+1,p.1+1)],
    }
}

fn diagonal_with_dirs(p: (isize, isize), d1: Dir, d2: Dir) -> Option<(isize, isize)> {
    if d1 == d2 { return None; }
    match (d1, d2) {
        (Dir::N, Dir::S) | (Dir::S, Dir::N) | (Dir::W, Dir::E) | (Dir::E, Dir::W) => None,
        (Dir::N, Dir::W) | (Dir::W, Dir::N) => Some((p.0-1,p.1-1)),
        (Dir::N, Dir::E) | (Dir::E, Dir::N) => Some((p.0-1,p.1+1)),
        (Dir::S, Dir::W) | (Dir::W, Dir::S) => Some((p.0+1,p.1-1)),
        (Dir::S, Dir::E) | (Dir::E, Dir::S) => Some((p.0+1,p.1+1)),
        _ => unreachable!(),
    }
}

fn count_corners(region: &HashSet<(isize, isize)>) -> usize {
    let mut corners = 0;

    for &p in region {
        let neighs: Vec<_> = neighbours(p).into_iter().filter(|(n,_)| region.contains(&n)).collect();
        corners += match neighs.len() {
            0 => 4, // Isolated point
            1 => 2, // Borders the exterior on 3 sides, thus it is a part of 2 corners
            2 => {
                let (d1, d2) = (neighs[0].1, neighs[1].1);
                if let Some(q) = diagonal_with_dirs(p, d1, d2) {
                    if region.contains(&q) {
                        1
                    } else {
                        2
                    }
                } else {
                    0
                }
            },
            3 => {
                // Surrounded on 3 sides
                // Check the corners on the sides it is surrounded
                let used_dirs: HashSet<_> = neighs.iter().map(|(_, d)| d).collect();
                let dir = Dir::all().into_iter().filter(|d| !used_dirs.contains(d)).next().unwrap();
                diagonals_with_dir(p, dir.op()).into_iter().filter(|q| !region.contains(&q)).count()
            },
            4 => diagonals(p).into_iter().filter(|q| !region.contains(&q)).count(), // Surrounded on all sides. It belongs to as many corners as exterior diagonal
                                                                                    // neighbours it has
            _ => unreachable!(),
        };
    }

    corners
}

fn run2(input: &str) -> usize {
    let map: HashMap<(isize, isize), char> = input.lines().enumerate().map(|(i, line)| line.chars().enumerate().map(|(j,c)| ((i as isize,j as isize), c)).collect::<Vec<_>>()).flatten().collect();

    let mut regions = Vec::new();
    let mut checked: HashSet<(isize, isize)> = HashSet::with_capacity(map.len());
    for (&p, c) in &map {
        if checked.contains(&p) { continue; }
        let mut region = HashSet::with_capacity(map.len());
        let mut queue = VecDeque::from([p]);
        region.insert(p);

        while let Some(p) = queue.pop_front() {
            for (n, _) in neighbours(p) {
                if !region.contains(&n) {
                    if map.get(&n).is_some_and(|d| c == d) {
                        region.insert(n);
                        queue.push_back(n);
                    }
                }
            }
        }

        checked = checked.union(&region).copied().collect();
        let area = region.len();
        regions.push((c, region, area));
    }

    regions.into_iter().map(|(_, region, area)| count_corners(&region) * area).sum()
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
fn example1p1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 140);
}

#[test]
fn example2p1() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 772);
}

#[test]
fn example3p1() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1930);
}

#[test]
fn inputp1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1377008);
}

#[test]
fn example1p2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 80);
}

#[test]
fn example3p2() {
    let input = fs::read_to_string("test3.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1206);
}

#[test]
fn example4p2() {
    let input = fs::read_to_string("test4.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 236);
}

#[test]
fn example5p2() {
    let input = fs::read_to_string("test5.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 368);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 815788);
}
