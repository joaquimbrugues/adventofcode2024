use std::{env,fs,process};


struct Computer<'a> {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instructions: &'a [u8],
    output: Vec<u8>,
}

impl<'a> Computer<'a> {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64, instructions: &'a [u8]) -> Self {
        Self { reg_a, reg_b, reg_c, instructions, output: vec![], }
    }

    fn resolve_combo(&self, code: u8) -> u64 {
        match code {
            0..=3 => code as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unexpected combo code {code}"),
        }
    }

    fn execute(&mut self) -> &Vec<u8> {
        // Initialize instruction pointer
        let mut inptr = 0;

        // Follow the program
        while inptr < self.instructions.len() {
            //for _ in 0..20 {
            let operand = self.instructions[inptr+1];
            match self.instructions[inptr] {
                // ADV
                0 => {
                    self.reg_a >>= self.resolve_combo(operand);
                    inptr += 2;
                },
                // BXL
                1 => {
                    self.reg_b ^= operand as u64;
                    inptr += 2;
                }
                // BST
                2 => {
                    self.reg_b = self.resolve_combo(operand) % 8;
                    inptr += 2;
                },
                // JNZ
                3 => {
                    if self.reg_a == 0 {
                        inptr += 2;
                    } else {
                        inptr = operand as usize;
                    }
                }
                // BXC
                4 => {
                    self.reg_b ^= self.reg_c;
                    inptr += 2;
                },
                // OUT
                5 => {
                    let out = self.resolve_combo(operand) % 8;
                    self.output.push(out as u8);
                    inptr += 2;
                },
                // BDV
                6 => {
                    self.reg_b = self.reg_a >> self.resolve_combo(operand);
                    inptr += 2;
                },
                //CDV
                7 => {
                    self.reg_c = self.reg_a >> self.resolve_combo(operand);
                    inptr += 2;
                }
                _ => unreachable!(),
            }
        }
        &self.output
    }
}

fn run1(input: &str) -> String {
    let (sreg, prog) = input.split_once("\n\n").unwrap();
    // Read the registry
    let mut registry = [0, 0, 0];
    for line in sreg.lines() {
        if let Some(s) = line.strip_prefix("Register A: ") {
            registry[0] = s.parse().unwrap();
        } else if let Some(s) = line.strip_prefix("Register B: ") {
            registry[1] = s.parse().unwrap();
        } else if let Some(s) = line.strip_prefix("Register C: ") {
            registry[2] = s.parse().unwrap();
        } else {
            panic!("Unexpected input: {line}");
        }
    }
    // Read the program
    let codes: Vec<u8> = prog.strip_prefix("Program: ").unwrap().trim()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .collect();
    let mut computer = Computer::new(registry[0], registry[1], registry[2], &codes);
    // Return
    computer.execute().into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",")
}

// FACTORS TO EXPLOIT
// 1- Each run of the program depends only on the 3 last digits of A (in both tests and our input)
// 2- There is exactly one jump instruction at the end, checking whether A is zero
//
// IDEA:
// Build possible A's by chunks, by producing candidates cummulatively matching increasing tail
// ends of the program input
fn run2(input: &str) -> u64 {
    let codes: Vec<u8> = input
        .split_once("\n\n").unwrap().1
        .strip_prefix("Program: ").unwrap().trim()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .collect();

    let mut min_a = u64::MAX;
    let mut candidates = (0..8).collect::<Vec<_>>();
    for cutoff in (0..codes.len()).rev() {
        let mut next_candidates = Vec::with_capacity(8 * candidates.len());
        for &a in &candidates {
            let mut  computer = Computer::new(a, 0, 0, &codes);
            let output = computer.execute();
            if *output == codes[cutoff..] {
                // This possible A yields all outputs from cutoff
                // Check if this is finished
                if cutoff == 0 {
                    min_a = std::cmp::min(min_a, a);
                } else {
                    // We still need to generate more outputs, add more candidates
                    (0..8).for_each(|k| next_candidates.push((a * 8) + k));
                }
            }
        }
        candidates = next_candidates;
    }

    min_a
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
    assert_eq!(res, String::from("4,6,3,5,6,3,5,2,1,0"));
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, String::from("3,1,4,3,1,7,1,6,3"));
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 117440);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 37221270076916);
}
