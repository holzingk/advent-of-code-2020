use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("Count 1 {}", day9_1());
    println!("Count 2 {}", day9_2());
}

fn day9_1() -> u64 {
    let xmas = parse("data/09");
    find_invalid(25, &xmas).to_owned()
}

fn day9_2() -> u64 {
    let xmas = parse("data/09");
    let invalid_number = find_invalid(25, &xmas).to_owned();
    let mut range = find_range_summing_to(invalid_number, &xmas);
    range.sort_unstable();
    range[0] + range[range.len() - 1]
}

fn parse(file_name: &str) -> Vec<u64> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse().unwrap()).collect()
}

fn find_invalid(preamble_len: u32, xmas: &[u64]) -> &u64 {
    for i in preamble_len as usize  .. xmas.len() - 2 {
	if !is_valid(&xmas[i - preamble_len as usize .. i + 1]) {
	    return &xmas[i]
	}
    }
    &0
}

fn is_valid(slice: &[u64]) -> bool {
    let target = &slice[slice.len() - 1];
    for i_a in 0 .. slice.len() - 1 {
	let a = &slice[i_a];
	for (i_b, b) in slice.iter().enumerate().take(slice.len() - 1) {
	    // do not sum the same number
	    if i_a == i_b {
		continue
	    }
	    //dbg!((&i_a, &a, &i_b, &b, &slice, &target, &slice.len()));
	    if *a + *b == *target { return true }
	}
    }
    false
}

fn find_range_summing_to(number: u64, xmas: &[u64]) -> Vec<u64> {
    for i in 0..xmas.len() - 1{
	let mut offset = 2;
	while i+offset < xmas.len() && xmas[i..i+offset].iter().sum::<u64>() <= number {
	    if xmas[i..i+offset].iter().sum::<u64>() == number {
		return xmas[i..i+offset].to_owned()
	    }
	    offset += 1;
	}
    }
    Vec::new()
}

#[test]
fn parser_test() {
    let xmas = parse("data/09_sample");
    assert_eq!(xmas[0], 35);
    assert_eq!(xmas[19], 576);
    assert_eq!(xmas.len(), 20);
}

#[test]
fn example_test() {
    let xmas = parse("data/09_sample");
    assert_eq!(find_invalid(5, &xmas), &127);
}

#[test]
fn results_test() {
    assert_eq!(day9_1(), 85848519);
    assert_eq!(day9_2(), 13414198);
}

#[test]
fn example_test2() {
    let xmas = parse("data/09_sample");
    let invalid_number = find_invalid(5, &xmas).to_owned();
    let range = find_range_summing_to(invalid_number, &xmas);
    assert_eq!(range, &[15, 25, 47, 40]);
}
