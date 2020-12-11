use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    println!("Count 1 {}", day10_1());
    println!("Count 2 {}", day10_2());
}

fn day10_1() -> u64 {
    let mut adapters = parse("data/10");
    task1(&mut adapters)
}

fn day10_2() -> u64 {
    let mut adapters = parse("data/10");
    adapters.sort_unstable();
    adapters.insert(0, 0);
    let ranges = split_at_three(&adapters);
    process_in_groups(&ranges)
}

fn task1(adapters: &mut [u64]) -> u64 {
    adapters.sort_unstable();
    let mut current_jolts = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;
    for adapter in adapters {
	if *adapter - current_jolts < 1 &&
	    *adapter - current_jolts > 3 {
		panic!("no suitable adapter found");
	    }
	if *adapter - current_jolts == 1 {
	    diff_1 += 1;
	} else if *adapter - current_jolts == 3 {
	    diff_3 += 1;
	}
	current_jolts = *adapter;
    }
    // last jolting:
    diff_3 += 1;
    diff_1 * diff_3
}

fn parse(file_name: &str) -> Vec<u64> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().parse().unwrap()).collect()
}

fn split_at_three(adapters: &[u64]) -> Vec<Vec<u64>> {
    let mut slices = Vec::new();
    let mut last = 0;
    let mut begin = 0;
    for (i, adapter) in adapters.iter().enumerate() {
	if adapter - last == 3 {
	    slices.push(adapters[begin..i+1].to_owned());
	    begin = i;
	}
	last = *adapter;
    }
    slices.push(adapters[begin..adapters.len()].to_owned());
    slices
}

fn check_range(range: &[u64]) -> bool {
    let mut last = range[0];
    for number in range {
	if number - last > 3 { return false }
	last = *number;
    }
    true
}

fn find_valid_combos(adapters: &[u64]) -> u64 {
    let mut ret = 0;
    for count in 1..adapters.len() {
	let combos: Vec<Vec<&u64>> = adapters[1..adapters.len() - 1].iter().to_owned().combinations(count).collect();
	for combo in combos {
	    let mut rest = (*adapters).to_owned();
	    rest.retain(|x| !combo.contains(&x));
	    if check_range(&rest) {
		ret += 1;
	    }
	}
    }
    ret + 1
}

fn process_in_groups(ranges: &[Vec<u64>]) -> u64 {
    let mut total = 1;
    for range in ranges {
	let combo_num = find_valid_combos(range);
	dbg!((&combo_num, &range));
	total *= combo_num;	
    }
    total
}

#[test]
fn parser_test() {
    let v = parse("data/10_sample_2");
    assert_eq!(v[0], 28);
    assert_eq!(v[1], 33);
    assert_eq!(v.len(), 31);
}

#[test]
fn sample_test_first() {
    let mut adapters = parse("data/10_sample");
    assert_eq!(task1(&mut adapters), 35);

    let mut adapters2 = parse("data/10_sample_2");
    assert_eq!(task1(&mut adapters2), 220);
}

#[test]
fn sample_test_second1() {
    let mut adapters = parse("data/10_sample");
    adapters.sort_unstable();
    adapters.insert(0, 0);
    let ranges = split_at_three(&adapters);
    
    assert_eq!(ranges[0], [0, 1, 4]);
    assert_eq!(ranges[1], [4, 5, 6, 7, 10]);
    assert_eq!(check_range(&vec![2, 4, 7]), true);
    assert_eq!(check_range(&vec![2, 4, 8]), false);
    assert_eq!(find_valid_combos(&vec![3, 4, 6, 8]), 2);
    assert_eq!(find_valid_combos(&vec![3, 5, 6, 7, 9]), 5);

    assert_eq!(process_in_groups(&ranges), 8);
}

#[test]
fn sample_test_second2() {
    let mut adapters = parse("data/10_sample_2");
    adapters.sort_unstable();
    adapters.insert(0, 0);
    dbg!(&adapters);
    let ranges = split_at_three(&adapters);
    dbg!(&ranges);
    assert_eq!(process_in_groups(&ranges), 19208);
}
