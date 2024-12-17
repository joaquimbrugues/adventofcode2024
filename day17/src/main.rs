use std::{env,fs,process};

fn resolve_combo(code: u8, reg: &[u32;3]) -> u32 {
    match code {
        0..=3 => code as u32,
        4 => reg[0],
        5 => reg[1],
        6 => reg[2],
        _ => panic!("Unexpected combo code {code}"),
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
    let prog = prog.strip_prefix("Program: ").unwrap().trim();
    let codes: Vec<u8> = prog.split(',').map(|tok| tok.parse().unwrap()).collect();
    // Initialize instruction pointer
    let mut inptr = 0;
    // Initialize output
    let mut output = String::new();

    // Follow the program
    while inptr < codes.len() {
    //for _ in 0..20 {
        let operand = codes[inptr+1];
        match codes[inptr] {
            // ADV
            0 => {
                registry[0] /= 2u32.pow(resolve_combo(operand, &registry));
                //println!("InPtr: {inptr}, ADV: A = {}", registry[0]);
                inptr += 2;
            },
            // BXL
            1 => {
                registry[1] ^= operand as u32;
                //println!("InPtr: {inptr}, BXL: B = {}", registry[1]);
                inptr += 2;
            }
            // BST
            2 => {
                registry[1] = resolve_combo(operand, &registry) % 8;
                //println!("InPtr: {inptr}, BST: B = {}", registry[1]);
                inptr += 2;
            },
            // JNZ
            3 => {
                //print!("InPtr: {inptr}, JNZ: ");
                if registry[0] == 0 {
                    inptr += 2;
                    //print!("A == 0, jump to {inptr}\n");
                } else {
                    inptr = operand as usize;
                    //print!("A != 0, jump to {inptr}\n");
                }
            }
            // BXC
            4 => {
                //print!("InPtr: {inptr}, BXC: B = {}, C = {}, ", registry[1], registry[2]);
                registry[1] ^= registry[2];
                //print!("res = {}\n", registry[1]);
                inptr += 2;
            },
            // OUT
            5 => {
                let out = resolve_combo(operand, &registry) % 8;
                if output.len() > 0 { output.push(','); }
                output.push(char::from_digit(out,10).unwrap());
                //println!("InPtr: {inptr}, OUT: out = {output}");
                inptr += 2;
            },
            // BDV
            6 => {
                registry[1] = registry[0] / 2u32.pow(resolve_combo(operand, &registry));
                //println!("InPtr: {inptr}, BDV: B = {}", registry[1]);
                inptr += 2;
            },
            //CDV
            7 => {
                registry[2] = registry[0] / 2u32.pow(resolve_combo(operand, &registry));
                //println!("InPtr: {inptr}, CDV: C = {}", registry[1]);
                inptr += 2;
            }
            _ => unreachable!(),
        }
    }

    // Return
    output
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

    let res = run1(&input);
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
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 117440);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
