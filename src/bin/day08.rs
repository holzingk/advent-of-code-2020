use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(PartialEq, Debug, Clone)]
enum Instr {
    Acc,
    Jmp,
    Nop,
}

fn main() {
   println!("Count 1 {}", day8_1());
   println!("Count 2 {}", day8_2());
}

fn day8_1() -> i32 {
    let program = parse();
    let (acc, _i, _trace) = trace(&program);
    acc
}

fn day8_2() -> i32 {
    let program = parse();
    trace2(&program).unwrap()
}

fn parse() -> Vec<(Instr, i32)> {
    let file = File::open("data/08").unwrap();
    let reader = BufReader::new(file);
    let mut program = Vec::new();
    for line in reader.lines() {
	let s = line.unwrap();
	let split_line: Vec<&str> = s.split_whitespace().collect();
	let instr = match split_line[0] {
	    "acc" => Instr::Acc,
	    "jmp" => Instr::Jmp,
	    "nop" => Instr::Nop,
	    &_ => panic!(),
	};
	let arg = split_line[1].parse().unwrap();
	program.push((instr, arg))
    }
    program
}

fn trace(program: &[(Instr, i32)]) -> (i32, i32, Vec<i32>) {
    let mut trace = Vec::new();
    let mut i = 0;
    let mut acc = 0;
    loop {
	let (new_acc, i_diff) = op(&program[i as usize].0, program[i as usize].1, acc);
	acc = new_acc;
	i += i_diff;
	if trace.contains(&i) {
	    break;
	}
	if i >= program.len() as i32 {
	    break;
	} else {
	    trace.push(i)
	}
    }
    (acc, i, trace)
}

fn trace2(program: &[(Instr, i32)]) -> Option<i32> {
    for (i, (instr, _)) in program.iter().enumerate() {
	let mut modded = program.to_owned();
	if instr == &Instr::Jmp {
	    modded[i as usize].0 = Instr::Nop;
	} else if instr == &Instr::Nop {
	    modded[i as usize].0 = Instr::Jmp;
	}
	let (acc, i, _trace) = trace(&modded);
	if i == program.len() as i32 {
	    return Some(acc);
	}
    }
    None
}

fn op(instr: &Instr, arg: i32, acc: i32) -> (i32, i32) {
    match instr {
	Instr::Acc => (acc + arg, 1),
	Instr::Jmp => (acc, arg),
	Instr::Nop => (acc, 1),
    }
}

#[test]
fn parser_test() {
    assert_eq!(parse()[0].0, Instr::Acc);
    assert_eq!(parse()[0].1, -5);
}

#[test]
fn result1_test() {
    assert_eq!(day8_1(), 1709);
}
