use std::{env,fs,process};

fn read_input(chunk: &str) -> [u32; 6] {
    let mut lines = chunk.lines();
    let (linea, lineb, linep) = (lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap());
    let linea = linea.strip_prefix("Button A: ").unwrap();
    let (ax, ay) = linea.split_once(", ").unwrap();
    let ax = ax.strip_prefix("X+").unwrap().parse().unwrap();
    let ay = ay.strip_prefix("Y+").unwrap().parse().unwrap();
    let lineb = lineb.strip_prefix("Button B: ").unwrap();
    let (bx, by) = lineb.split_once(", ").unwrap();
    let bx = bx.strip_prefix("X+").unwrap().parse().unwrap();
    let by = by.strip_prefix("Y+").unwrap().parse().unwrap();
    let linep = linep.strip_prefix("Prize: ").unwrap();
    let (px, py) = linep.split_once(", ").unwrap();
    let px = px.strip_prefix("X=").unwrap().parse().unwrap();
    let py = py.strip_prefix("Y=").unwrap().parse().unwrap();

    [ax, ay, bx, by, px, py]
}

fn gcd(mut a: f64, mut b: f64) -> f64 {
    while b > 0.0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn create_system(ints: [u32; 6], part2: bool) -> [f64; 6] {
    let syst = [
        ints[0].into(),
        ints[1].into(),
        ints[2].into(),
        ints[3].into(),
        if part2 { (ints[4] as f64) + 10_000_000_000_000.0} else { ints[4].into() },
        if part2 { (ints[5] as f64) + 10_000_000_000_000.0} else { ints[5].into() },
    ];
    let gx = gcd(gcd(syst[0], syst[2]), syst[4]);
    let gy = gcd(gcd(syst[1], syst[3]), syst[5]);
    [ syst[0] / gx, syst[1] / gy, syst[2] / gx, syst[3] / gy, syst[4] / gx, syst[5] / gy, ]
}

fn resolve_system(sys: [f64; 6]) -> u64 {
    let det = (sys[0] * sys[3]) - (sys[1] * sys[2]);
    if det == 0.0 {
        panic!("Determinant is 0!");
    }
    let x = ( sys[3] * sys[4] - sys[2] * sys[5] ) / det;
    let y = ( sys[0] * sys[5] - sys[1] * sys[4] ) / det;

    if x == x.floor() && y == y.floor() {
        3u64 * (x as u64) + (y as u64)
    } else {
        0
    }
}

fn run1(input: &str) -> u64 {
    input.split("\n\n").map(|chunk| resolve_system(create_system(read_input(chunk), false))).sum()
}

fn run2(input: &str) -> u64 {
    input.split("\n\n").map(|chunk| resolve_system(create_system(read_input(chunk), true))).sum()
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
    assert_eq!(res, 480);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 27157);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 875318608908);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 104015411578548);
}
