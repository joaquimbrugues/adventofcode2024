use std::{env,fs,process};
use std::collections::{HashMap,HashSet,BTreeSet,VecDeque};

fn parse_input(input: &str) -> HashMap<&str, BTreeSet<&str>> {
    let mut res = HashMap::with_capacity(input.lines().count());
    for line in input.lines() {
        let (s1, s2) = line.split_once('-').unwrap();
        res.entry(s1).or_insert(BTreeSet::new()).insert(s2);
        res.entry(s2).or_insert(BTreeSet::new()).insert(s1);
    }

    res
}

fn run1(input: &str) -> usize {
    let graph = parse_input(input);
    let mut cycles = HashSet::new();

    graph.iter().for_each(|(k, nns)| {
        if k.starts_with('t') {
            for (i, n1) in nns.iter().enumerate() {
                for (_, n2) in nns.iter().enumerate().skip_while(|(j, _)| *j <= i) {
                    if graph.get(n1).unwrap().contains(n2) {
                        cycles.insert(BTreeSet::from([k, n1, n2]));
                    }
                }
            }
        }
    });
    cycles.len()
}

fn run2(input: &str) -> String {
    let graph = parse_input(input);
    let mut three_cycles = HashSet::new();
    let mut best_clique = None;
    graph.iter().for_each(|(k, nns)| {
        for (i, n1) in nns.iter().enumerate() {
            for (_, n2) in nns.iter().enumerate().skip_while(|(j, _)| *j <= i) {
                if graph.get(n1).unwrap().contains(n2) {
                    if best_clique.is_none() {
                        best_clique = Some(BTreeSet::from([k, n1, n2]));
                    }
                    three_cycles.insert(BTreeSet::from([k, n1, n2]));
                }
            }
        }
    });

    let mut best_clique = best_clique.unwrap();
    three_cycles.into_iter().for_each(|cycle| {
        // All nodes in a clique better that best_clique with length `n` must have degree strictly greater that `n - 1`
        if cycle.iter().all(|&n| graph.get(n).unwrap().len() >= best_clique.len()) {
            // Search for a better clique
            let mut clique = cycle.clone();
            let mut seen: HashSet<_> = cycle.iter().copied().collect();
            let mut queue: VecDeque<_> = cycle.into_iter().collect();
            while let Some(n) = queue.pop_front() {
                for nn in graph.get(n).unwrap() {
                        // Do not repeat checks (BFS)
                    if seen.insert(nn)
                        // Same condition as before: every node in the clique must have a minimum
                        // degree
                        && graph.get(nn).unwrap().len() >= best_clique.len()
                        // Check if the node belongs to the clique
                        && clique.iter().all(|&node| graph.get(node).unwrap().contains(nn))
                    {
                        clique.insert(nn);
                        queue.push_back(nn);    // Keep searching
                    }
                }
            }
            if clique.len() > best_clique.len() {
                best_clique = clique;
            }
        }
    });

    best_clique.into_iter().fold(String::new(), |mut acc, node| {
        if acc.len() > 0 { acc.push(','); }
        acc.push_str(node);
        acc
    })
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
    assert_eq!(res, 7);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 1308);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, String::from("co,de,ka,ta"));
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, String::from("bu,fq,fz,pn,rr,st,sv,tr,un,uy,zf,zi,zy"));
}
