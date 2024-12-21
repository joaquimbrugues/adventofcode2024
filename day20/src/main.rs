use std::{env,fs,process};
use std::collections::{HashSet,HashMap,VecDeque};

fn parse_input(input: &str) -> (HashSet<(isize, isize)>, (isize, isize), (isize, isize)) {
    let mut walkable = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.lines().enumerate() {
        let y = y as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            match c {
                '.' => { walkable.insert((y,x)); },
                '#' => {},
                'S' => {
                    if start.is_none() {
                        start = Some((y,x));
                        walkable.insert((y,x));
                    } else {
                        panic!("Start position assigned twice!");
                    }
                },
                'E' => {
                    if end.is_none() {
                        end = Some((y,x));
                        walkable.insert((y,x));
                    } else {
                        panic!("End position assigned twice!");
                    }
                },
                _ => unreachable!(),
            }
        }
    }

    (walkable, start.unwrap(), end.unwrap())
}

fn neighbours(point: (isize, isize)) -> [(isize, isize); 4] {
    [
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
    ]
}

fn bfs(walkable: &HashSet<(isize, isize)>, init: (isize, isize)) -> HashMap<(isize, isize), u32> {
    let mut queue = VecDeque::from([(init, 0)]);
    let mut dists = HashMap::from([(init, 0)]);

    while let Some((point, dist)) = queue.pop_front() {
        for n in neighbours(point) {
            if walkable.contains(&n) && !dists.contains_key(&n) {
                dists.insert(n, dist+1);
                queue.push_back((n, dist+1));
            }
        }
    }

    dists
}

fn compute_cheats(walkable: &HashSet<(isize, isize)>, start: (isize, isize), distances_to_end: &HashMap<(isize, isize), u32>, max_distance: u32, cheat_len: u32) -> HashMap<((isize, isize), (isize, isize)), u32> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);

    let mut cheat_savings = HashMap::new();
    while let Some((point, dist)) = queue.pop_front() {
        if dist + cheat_len < max_distance {
            for n in neighbours(point).into_iter().filter(|n| walkable.contains(n)) {
                if visited.insert(n) { queue.push_back((n, dist+1)); }
            }
            // Scan a diamond-shaped region around point, governed by the equation |x - point.1| +
            // |y - point.0| <= cheat_len
            let radius = cheat_len as isize;
            for y in (point.0 - radius)..=(point.0 + radius) {
                let width = radius - (y - point.0).abs();
                for x in (point.1 - width)..=(point.1 + width) {
                    let dd = ((y - point.0).abs() + (x - point.1).abs()) as u32;
                    if let Some(rest_dis) = distances_to_end.get(&(y,x)) {
                        if dist + dd + rest_dis < max_distance {
                            cheat_savings.insert((point, (y,x)), max_distance - (dist + dd + rest_dis));
                        }
                    }
                }
            }
        }
    }
    cheat_savings
}

fn run1(input: &str, minimum_picoseconds_saved: u32) -> usize {
    let (walkable, start, end) = parse_input(input);
    let dists = bfs(&walkable, end);
    let no_cheat_time = *dists.get(&start).unwrap();
    let cheats = compute_cheats(&walkable, start, &dists, no_cheat_time, 2);
    cheats.values().filter(|&&time| time >= minimum_picoseconds_saved).count()
}

fn run2(input: &str, minimum_picoseconds_saved: u32) -> usize {
    let (walkable, start, end) = parse_input(input);
    let dists = bfs(&walkable, end);
    let no_cheat_time = *dists.get(&start).unwrap();
    let cheats = compute_cheats(&walkable, start, &dists, no_cheat_time, 20);
    cheats.values().filter(|&&time| time >= minimum_picoseconds_saved).count()
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

    let res = run2(&input,100);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input,0);
    assert_eq!(res, 44);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input,100);
    assert_eq!(res, 1521);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input, 50);
    assert_eq!(res, 285);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input, 100);
    assert_eq!(res, 1013106);
}
