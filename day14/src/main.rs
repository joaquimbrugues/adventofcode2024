use std::{env,fs,process};
use std::collections::{HashMap,HashSet};

struct LazyRobot {
    current_pos: (isize, isize),
    height: isize,
    width: isize,
    vel: (isize, isize),
    positions: HashMap<(isize, isize), (isize, isize)>,
}

impl LazyRobot {
    fn new(current_pos: (isize, isize), (height, width): (isize, isize), vel: (isize, isize)) -> Self {
        Self { current_pos, height, width, vel, positions: HashMap::new(), }
    }

    fn walk(&mut self) {
        self.current_pos = *self.positions.entry(self.current_pos).or_insert_with(|| {
            let mut np = (self.current_pos.0 + self.vel.0, self.current_pos.1 + self.vel.1);
            while np.0 < 0 { np.0 += self.height; }
            np.0 %= self.height;
            while np.1 < 0 { np.1 += self.width; }
            np.1 %= self.width;
            np
        })
    }
}

fn parse_robot(line: &str, height: isize, width: isize) -> LazyRobot {
    let (first, second) = line.split_once(' ').unwrap();
    let first = first.strip_prefix("p=").unwrap();
    let (sx, sy) = first.split_once(',').unwrap();
    let pos  = (sy.parse().unwrap(), sx.parse().unwrap());
    let second = second.strip_prefix("v=").unwrap();
    let (sx, sy) = second.split_once(',').unwrap();
    let vel = (sy.parse().unwrap(), sx.parse().unwrap());
    LazyRobot::new(pos, (height, width), vel)
}

fn run1(input: &str, height: isize, width: isize) -> usize {
    let mut quadrants = [0; 4]; // In order: top-left, top-right, bottom-left, bottom-right
    input.lines().filter(|line| !line.starts_with("//")).for_each(|line| {
        let mut robot = parse_robot(line, height, width);

        for _ in 0..100 {
            robot.walk();
        }

        if robot.current_pos.0 < height / 2 {
            if robot.current_pos.1 < width / 2 {
                quadrants[0] += 1;
            } else if robot.current_pos.1 > width / 2 {
                quadrants[1] += 1;
            }
        } else if robot.current_pos.0 > height / 2 {
            if robot.current_pos.1 < width / 2 {
                quadrants[2] += 1;
            } else if robot.current_pos.1 > width / 2 {
                quadrants[3] += 1;
            }
        }
    });
    quadrants.into_iter().product()
}

fn print_map(map: &HashSet<(usize, usize)>, height: isize, width: isize) {
    let height = height as usize;
    let width = width as usize;

    for y in 0..height {
        for x in 0..width {
            if map.contains(&(y,x)) {
                print!("🤖");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
}

const LEN_TO_CHECK: usize = 7;

fn run2(input: &str, height: isize, width: isize) -> usize {
    let mut robots: Vec<_> = input.lines().map(|line| parse_robot(line, height, width)).collect();
    let mut second = 0;
    loop {
        second += 1;
        robots.iter_mut().for_each(|r| r.walk());
        let positions: HashSet<_> = robots.iter().map(|r| (r.current_pos.0 as usize, r.current_pos.1 as usize)).collect();
        for y in 0..((height as usize) - LEN_TO_CHECK) {
            for x in 0..((width as usize) - LEN_TO_CHECK) {
                if (0..LEN_TO_CHECK).all(|z| positions.contains(&(y+z, x)) && positions.contains(&(y, x+z))) {
                    print_map(&positions, height, width);
                    return second;
                }
            }
        }
    }
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

    //let res = run1(&input, 7, 11);
    let res = run2(&input, 103, 101);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 7, 11);
    assert_eq!(res, 12);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input, 103, 101);
    assert_eq!(res, 219150360);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input, 103, 101);
    assert_eq!(res, 8053);
}
