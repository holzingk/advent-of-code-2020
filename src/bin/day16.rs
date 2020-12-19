use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
struct Input {
    rules: HashMap<String, (u32, u32, u32, u32)>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn main() {
    println!("Count 1 {}", day16_1("data/16"));
    println!("Count 2 {}", day16_2("data/16"));
} 

fn day16_1(file_name: &str) -> u32 {
    let input = parse(&file_name);
    let mut allowed = vec![false; 1000];
    for (lower_first, upper_first, lower_second, upper_second) in input.rules.values() {
	for i in
	    (*lower_first..*upper_first + 1)
	    .chain(*lower_second..*upper_second + 1)
	{
	    allowed[i as usize] = true;
	}
    }
    let mut res = 0;
    for nearby_ticket in &input.nearby_tickets {
	for val in nearby_ticket {
	    if !allowed[*val as usize] {
		res += *val;
	    }
	}
    }
    res
}

fn day16_2(file_name: &str) -> u64 {
    let mut input = parse(&file_name);
    let mut allowed = vec![false; 1000];
    for (lower_first, upper_first, lower_second, upper_second) in input.rules.values() {
	for i in
	    (*lower_first..*upper_first + 1)
	    .chain(*lower_second..*upper_second + 1)
	{
	    allowed[i as usize] = true;
	}
    }

    input.nearby_tickets
	.retain(|fields| fields.iter().all(|field| allowed[*field as usize]));
    //dbg!(&input);
    
    let exclusion_map: HashMap<String, RefCell<Vec<u32>>> = input.rules
	.iter()
	.map(|(k,_v)| (k.clone(), RefCell::new(Vec::new())))
	.collect();
    for column in 0..input.nearby_tickets[0].len() {
	let mut numbers = Vec::new();
	for row in &input.nearby_tickets {
	    numbers.push(row[column]);
	}
	for (name, limits) in &input.rules {
	    if !numbers.iter().all(|n| (n >= &limits.0 && n <= &limits.1) ||
				  (n >= &limits.2 && n <= &limits.3)) {
		exclusion_map.get(name).unwrap().borrow_mut().push(column as u32);
		//dbg!(&name, &limits, &numbers);
		//println!("Add candidate {} for rule {} to exclusion list", column, name);
	    }
	}
    }
    //dbg!(&exclusion_map);

    let mut relation = HashMap::new();
    while let Some((name, numbers)) =  exclusion_map
	.iter()
	.find(|(_k,v)| v.borrow().len() == input.nearby_tickets[0].len() - 1)
    {
	for number in 0..input.nearby_tickets[0].len() {
	    if !numbers.borrow().contains(&(number as u32)) {
		relation.insert(name, number);
		break
	    }
	}
	//dbg!(&relation);
	for v in relation.values() {
	    for (_name, list) in exclusion_map.iter() {
		if !list.borrow().contains(&(*v as u32)) {
		    list.borrow_mut().push(*v as u32);
		}
	    }
	}
	//dbg!(&exclusion_map);
    }

    let mut res = 1;
    for (k, v) in &relation {
	if k.starts_with("departure") {
	    res *= input.my_ticket[*v as usize] as u64;
	}
    }
    res
}

fn parse(file_name: &str) -> Input {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut rules = HashMap::new();
    // fields
    let re_fields = Regex::new(
	r"((?:\w+ \w+)|(?:\w+)): (\d+)-(\d+) or (\d+)-(\d+)")
	.unwrap();
    for line in &mut lines {
	let s = line.unwrap();
	if s.is_empty() {
	    break
	}
	let field_captures = re_fields.captures(&s).unwrap();
	let field_name = field_captures.get(1).unwrap().as_str().parse().unwrap();
	let lower_first = field_captures.get(2).unwrap().as_str().parse().unwrap();
	let upper_first = field_captures.get(3).unwrap().as_str().parse().unwrap();
	let lower_second = field_captures.get(4).unwrap().as_str().parse().unwrap();
	let upper_second = field_captures.get(5).unwrap().as_str().parse().unwrap();
	rules.insert(field_name,
		     (lower_first, upper_first, lower_second, upper_second));
	
    }

    // my ticket
    let mut throwaway = lines.next().unwrap().unwrap();
    assert_eq!(throwaway, "your ticket:");
    let my_ticket: Vec<u32> = lines.next().unwrap().unwrap()
	.split(',')
	.map(|s| s.parse().unwrap())
	.collect();
    lines.next();
    
    // nearby tickets
    throwaway = lines.next().unwrap().unwrap();
    assert_eq!(throwaway, "nearby tickets:");
    let mut nearby_tickets = Vec::new();
    for line in lines {
	let s = line.unwrap();
	let fields: Vec<u32> = s
	    .split(',')
	    .map(|s| s.parse().unwrap())
	    .collect();
	nearby_tickets.push(fields);
    }
    let len_first = nearby_tickets[0].len();
    assert!(nearby_tickets.iter().all(|v| len_first == v.len()));
    Input { rules, my_ticket, nearby_tickets }
}

#[test]
fn parser_test() {
    let res = parse("data/16");
    dbg!(&res);
    //    res.rules
}

#[test]
fn assignment_1() {
    assert_eq!(day16_1("data/16_sample"), 71);
    assert_eq!(day16_1("data/16"), 23054);
}


#[test]
fn assignment_2() {
    assert_eq!(day16_2("data/16"), 51240700105297);
}
