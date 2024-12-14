use std::{env,fs,process};
use std::collections::HashMap;

struct LazyRobot {
    height: isize,
    width: isize,
    vel: (isize, isize),
    positions: HashMap<(isize, isize), (isize, isize)>,
}

impl LazyRobot {
    fn new((height, width): (isize, isize), vel: (isize, isize)) -> Self {
        Self { height, width, vel, positions: HashMap::new(), }
    }

    fn walk(&mut self, pos: (isize, isize)) -> (isize, isize) {
        *self.positions.entry(pos).or_insert_with(|| {
            let mut np = (pos.0 + self.vel.0, pos.1 + self.vel.1);
            while np.0 < 0 { np.0 += self.height; }
            np.0 %= self.height;
            while np.1 < 0 { np.1 += self.width; }
            np.1 %= self.width;
            np
        })
    }
}

fn run1(input: &str, height: isize, width: isize) -> usize {
    let mut quadrants = [0; 4]; // In order: top-left, top-right, bottom-left, bottom-right
    input.lines().filter(|line| !line.starts_with("//")).for_each(|line| {
        let (first, second) = line.split_once(' ').unwrap();
        let first = first.strip_prefix("p=").unwrap();
        let (sx, sy) = first.split_once(',').unwrap();
        let mut pos  = (sy.parse().unwrap(), sx.parse().unwrap());
        let second = second.strip_prefix("v=").unwrap();
        let (sx, sy) = second.split_once(',').unwrap();
        let vel = (sy.parse().unwrap(), sx.parse().unwrap());
        let mut robot = LazyRobot::new((height, width), vel);

        for _ in 0..100 {
            pos = robot.walk(pos);
        }

        if pos.0 < height / 2 {
            if pos.1 < width / 2 {
                quadrants[0] += 1;
            } else if pos.1 > width / 2 {
                quadrants[1] += 1;
            }
        } else if pos.0 > height / 2 {
            if pos.1 < width / 2 {
                quadrants[2] += 1;
            } else if pos.1 > width / 2 {
                quadrants[3] += 1;
            }
        }
    });
    println!("{quadrants:?}");
    quadrants.into_iter().product()
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

    //let res = run1(&input, 7, 11);
    let res = run1(&input, 103, 101);
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

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
