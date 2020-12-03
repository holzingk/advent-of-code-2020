use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("./data/01").unwrap();
    let reader = BufReader::new(file);
    let numbers: Vec<i32>  = reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect();

    println!("Answer 1: {}", star_one(&numbers).unwrap());
    println!("Answer 2: {}", star_two(&numbers).unwrap());
}

fn star_one(numbers: &[i32]) -> Result<i32, &'static str> {
    for number_1 in &*numbers {
	for number_2 in &*numbers {
	    if number_1 + number_2 == 2020 {
		return Ok(number_1 * number_2);
	    }
	}
    }
    Err("Combination not found")
}

fn star_two(numbers: &[i32]) -> Result<i32, &'static str> {
    for number_1 in &*numbers {
	for number_2 in &*numbers {
	    for number_3 in &*numbers {
		if number_1 + number_2 + number_3 == 2020 {
		    return Ok(number_1 * number_2 * number_3);
		}
	    }
	}
    }
    Err("Combination not found")
}
