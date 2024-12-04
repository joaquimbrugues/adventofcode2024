use std::{env,fs,process};

#[derive(Debug)]
enum Dir { N, NE, E, SE, S, SW, W, NW, }

impl Dir {
    fn all() -> [Self; 8] {
        [
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }

    fn next(&self, coord: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::N => {
                if coord.0 > 0 {
                    Some((coord.0 - 1, coord.1))
                } else {
                    None
                }
            },
            Self::NE => {
                if coord.0 > 0 {
                    Some((coord.0 - 1, coord.1 + 1))
                } else {
                    None
                }
            },
            Self::E => Some((coord.0, coord.1 + 1)),
            Self::SE => Some((coord.0 + 1, coord.1 + 1)),
            Self::S => Some((coord.0 + 1, coord.1)),
            Self::SW => {
                if coord.1 > 0 {
                    Some((coord.0 + 1, coord.1 - 1))
                } else {
                    None
                }
            },
            Self::W => {
                if coord.1 > 0 {
                    Some((coord.0, coord.1 - 1))
                } else {
                    None
                }
            },
            Self::NW => {
                if coord.0 > 0 && coord.1 > 0 {
                    Some((coord.0 - 1, coord.1 - 1))
                } else {
                    None
                }
            }
        }
    }
}

fn run1(input: &str) -> u32 {
    let mut soup = vec![];
    let mut stack = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == 'X' {
                stack.push(((i,j),0,None));
            }
            row.push(c);
        }
        soup.push(row);
    }

    let mut counter = 0;
    let xmas = ['X', 'M', 'A', 'S'];

    while let Some((coord, depth, dir)) = stack.pop() {
        if coord.0 < soup.len() && coord.1 < soup[coord.0].len() && xmas[depth] == soup[coord.0][coord.1] {
            if depth == 3 {
                counter += 1;
            } else if depth == 0 {
                assert!(dir.is_none());
                for d in Dir::all() {
                    if let Some(next) = d.next(coord) {
                        stack.push((next, 1, Some(d)));
                    }
                }
            } else {
                let dir = dir.unwrap();
                if let Some(next) = dir.next(coord) {
                    stack.push((next, depth + 1, Some(dir)));
                }
            }
        }
    }
    
    counter
}

fn run2(input: &str) -> u32 {
    let mut soup = vec![];
    let mut a_s = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == 'A' {
                a_s.push((i,j));
            }
            row.push(c);
        }
        soup.push(row);
    }

    let height = soup.len();
    let width = soup[0].len();

    a_s.into_iter().filter(|&(i,j)| i > 0 && j > 0 && i < height - 1 && j < width - 1).map(|(i,j)| {
        if (( soup[i-1][j-1] == 'S' && soup[i+1][j+1] == 'M' ) ||
            ( soup[i-1][j-1] == 'M' && soup[i+1][j+1] == 'S' )) &&
            (( soup[i-1][j+1] == 'S' && soup[i+1][j-1] == 'M' ) ||
             ( soup[i-1][j+1] == 'M' && soup[i+1][j-1] == 'S' )) {
                1
            } else {
                0
        }
    }).sum()
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
    assert_eq!(res, 18);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2549);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 9);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2003);
}
