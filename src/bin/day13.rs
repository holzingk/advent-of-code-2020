use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("Count 1 {}", day13_1("data/13"));
    println!("Count 2 {}", day13_2("data/13"));
}

fn day13_1(file_name: &str) -> u32 {
    let (timestamp, lines) = parse(&file_name);
    let mut nexts = Vec::new();
    for (line_i, line) in lines.iter().enumerate() {
	let mut next = timestamp as f32 / *line as f32;
	next = next.ceil() * lines[line_i as usize] as f32;
	assert!(next >= timestamp as f32);
	nexts.push((line_i as usize, next as u32));	
    }
    let (i, next) = nexts.iter().min_by_key(|(_i,v)| v).unwrap();
    let waittime = next - timestamp;
    let line_number = lines[*i];
    waittime * line_number
}


fn day13_2(file_name: &str) -> u64 {
    let lines = parse2(&file_name);
    //brute_force(&lines)
    let mut residues = Vec::new();
    for (line_i, line) in lines.iter().enumerate() {
	if line.is_some() {
	    residues.push(line.unwrap() as i64 - line_i as i64);
	}
    }
    let mut moduli = Vec::new();
    for (_line_i, line) in lines.iter().enumerate() {
	if line.is_some() {
	    moduli.push(line.unwrap() as i64);
	}
    }

    match chinese_remainder(&residues, &moduli) {
	Some(n) => n as u64,
	None => panic!("pitty"),
    }
}

#[allow(dead_code)]
fn brute_force(lines: &[Option<u64>]) -> u64 {
    let mut numbers = Vec::new();
    for (line_i, line) in lines.iter().enumerate() {
	if line.is_some() {
	    numbers.push((line_i, line.unwrap()));
	}
    }
    let mut numbers_sorted = numbers.to_owned();
    numbers_sorted.sort_by_key(|(_i, v)| *v);
    let (max_i, max) = numbers_sorted[numbers_sorted.len() - 1];
    let mut cur_max: u64 = max;
    let mut i: u64 = 0;
    loop {
	let mut found = false;	
	for (number_i, number) in &numbers {
	    let other_number = cur_max as i64 - (max_i as i64 - *number_i as i64);
	    if other_number % *number as i64 != 0 {
		found = false;
		break;
	    }
	    found = true;
	}
	if found  {
	    return cur_max - (max_i - numbers[0].0) as u64
	}
	cur_max += max;
	if i % 100000 == 0 {
	    print!("beep")
	}
	i += 1;
    }
    
}

// from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
#[allow(clippy::many_single_char_names)] 
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}


fn parse(file_name: &str) -> (u32, Vec<u32>) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let timestamp = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
    let lines = lines.next().unwrap().unwrap()
	.split(',')
	.map(|s| s.parse::<u32>())
	.filter(|o| o.is_ok())
	.map(|o| o.unwrap())
	.collect();
    (timestamp, lines)
}

fn parse2(file_name: &str) -> Vec<Option<u64>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let _timestamp = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
    let lines = lines.next().unwrap().unwrap()
	.split(',')
	.map(|s| s.parse::<u64>())
	.map(|o| match o {
	    Ok(x) => Some(x),
	    _ => None
	})
	.collect();
    lines
}


#[test]
fn parse_test() {
    let (timestamp, lines) = parse("data/13_sample");
    assert_eq!(timestamp, 939);
    assert_eq!(lines, [7, 13, 59, 31, 19]);
}

#[test]
fn assignment_1() {
    assert_eq!(day13_1("data/13_sample"), 295);
    assert_eq!(day13_1("data/13"), 2238);
}

#[test]
fn assignment_2() {
    assert_eq!(day13_2("data/13_sample"), 1068781);
    assert_eq!(day13_2("data/13"), 560214575859998);
}
