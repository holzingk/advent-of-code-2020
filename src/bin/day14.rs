use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    mask: Vec<Option<bool>>,
    memset: Vec<(u64, u64)>,
}

fn main() {
    println!("Count 1 {}", day14_1("data/14"));
    println!("Count 2 {}", day14_2("data/14"));
} 

fn day14_1(file_name: &str) -> u64 {
    let program = parse(&file_name);
    let mut memory = HashMap::new();
    for entry in &program {
	for (address, value) in &entry.memset {
	    let mut new_value: u64 = *value;
	    for (i, bit) in entry.mask.iter().enumerate() {
		match bit {
		    None => { },
		    Some(true) => { // set to one
			new_value |= 1 << (35 - i);
		    },
		    Some(false) => { // set to zero
			new_value &= !(1 << (35 - i));
		    }
		}
	    }
	    memory.insert(address, new_value); 
	}
    }
    let mut ret = 0;
    for (_k, v) in memory.iter() {
	ret += *v;
    }
    ret
}

fn day14_2(file_name: &str) -> u64 {
    
    let memory = day14_21(&file_name);
    let mut ret = 0;
    for (_k, v) in memory.iter() {
	ret += v;
    }
    ret
}

fn day14_21(file_name: &str) -> HashMap<u64, u64> {
    let program = parse(&file_name);
    let mut memory = HashMap::new();
    for entry in program {
	for (address, value) in entry.memset {
	    let mut new_address: u64 = address;
	    let mut floating: Vec<usize> = Vec::new();
	    for (i, bit) in entry.mask.iter().enumerate() {
		match bit {
		    None => {
			floating.push(i);
		    },
		    Some(true) => { // set to one
			new_address |= 1 << (35 - i);
		    },
		    Some(false) => {},
		}
	    }
	    for i in 0..(2_u64.pow(floating.len() as u32)) {
		let mut new_address2 = new_address;
		for (j, float) in floating.iter().enumerate() {
		    // if bit from i at position j is set
		    if i & (1 << j) != 0 {
			// set floatig bit
			new_address2 |= 1 << (35 - float);
		    } else {
			// unset floating bit
			new_address2 &= !(1 << (35 - float));
		    }
		}
		memory.insert(new_address2, value);
	    }
	}
    }
    memory
}

 
fn parse(file_name: &str) -> Vec<Entry> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let re_mask = Regex::new(r"mask = ([01X]+)").unwrap();
    let re_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut entries = Vec::new();
    let mut mask = Vec::new();
    let mut mems = Vec::new();
    for line in lines {
	let s = line.unwrap();
	let captures_mask = re_mask.captures(&s);
	if let Some(cm) = captures_mask {
	    if !mems.is_empty() {
		entries.push(Entry { mask, memset: mems });
	    }
	    mems = Vec::new();
	    mask = cm.get(1).unwrap()
		.as_str()
		.chars()
		.map(|c| match c {
		    'X' => None,
		    '1' => Some(true),
		    '0' => Some(false),
		    _ => panic!("parsing error"),
		})
		.collect();
	    assert_eq![mask.len(), 36];
	} else {
	    let captures_mem = re_mem.captures(&s).unwrap();
	    mems.push((captures_mem.get(1).unwrap().as_str().parse().unwrap(),
			captures_mem.get(2).unwrap().as_str().parse().unwrap()));
	}
    }
    entries.push(Entry { mask, memset: mems });
    entries
}

#[test]
fn paser_test() {
    let res = parse("data/14_sample");
    dbg!(&res);
    assert_eq!(res[0].mask[0], None);
    assert_eq!(res[0].mask[30], None);
    assert_eq!(res[0].mask[34], Some(false));
    assert!(res[0].memset.contains(&(8, 0)));
    assert!(res[0].memset.contains(&(7, 101)));

    let res2 = parse("data/14");
    dbg!(&res2);
    assert_eq!(res2[1].mask[0], Some(true));
    assert!(res2[1].memset.contains(&(60704, 43881206)));
}

#[test]
fn assignments() {
    assert_eq!(day14_1("data/14_sample"), 165);
    assert_eq!(day14_2("data/14_sample_2"), 208);
    assert_eq!(day14_2("data/14"), 4275496544925);
}
