use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

static REGEX_EXPRESSION: &str = r"(\d+)-(\d+) ([a-z]): ([a-z]+)";

fn main() {
    println!("Count 1 {}", day2_1());
    println!("Count 2 {}", day2_2());
}

fn day2_1() -> u32 {
    let file = File::open("./data/02").unwrap();
    let reader = BufReader::new(file);
    let count: u32 = reader.lines()
	.map(|s| s.unwrap())
	.filter(|s| check_line(&s))
	.count() as u32;
    count
}

fn day2_2() -> u32 {
    let file = File::open("./data/02").unwrap();
    let reader = BufReader::new(file);
    let count: u32 = reader.lines()
	.map(|s| s.unwrap())
	.filter(|s| check_line_two(&s))
	.count() as u32;
    count
}
    
fn check_line(line: &str) -> bool {
    let (lower, upper, pivot, password) = parse_line(line);
    //dbg!(lower, upper, pivot, password);
    let count: u32 = password.chars()
	.filter(|c| *c == pivot)
	.count() as u32;
//    dbg!(count);
    if count >= lower && count <= upper {
	return true
    }
    false
}

fn check_line_two(line: &str) -> bool {
    let (lower, upper, pivot, password) = parse_line(line);
    //dbg!(lower, upper, pivot, password);
    let pchars: Vec<char> = password.chars().collect();
//    if lower < pchars_len -  && upper < pchars_len - 1 {
    if (pchars[lower as usize - 1] == pivot) ^ (pchars[upper as usize - 1] == pivot) {
	    return true
	}
//    }
    false
}

fn parse_line(line: &str) -> (u32, u32, char, &str) {
    let re = Regex::new(REGEX_EXPRESSION).unwrap();
    let captures = re.captures(line).unwrap();
    let lower: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let upper: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let pivot: char = captures.get(3).unwrap().as_str().chars().next().unwrap();
    let password: &str = captures.get(4).unwrap().as_str();
    (lower, upper, pivot, password)
}

#[test]
fn test_rex() {
    let re = Regex::new(REGEX_EXPRESSION).unwrap();
    let text = "7-9 l: vslmtglbc";
    let captures = re.captures(text).unwrap();
    assert_eq!(captures.get(1).unwrap().as_str(), "7");	
    assert_eq!(captures.get(2).unwrap().as_str(), "9");
    assert_eq!(captures.get(3).unwrap().as_str(), "l");
    assert_eq!(captures.get(4).unwrap().as_str(), "vslmtglbc");	
}

#[test]
fn test_cookies() {
    assert_eq!(day2_1(), 614);
    assert_eq!(day2_2(), 354);
}
    
