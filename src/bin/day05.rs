use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    println!("Count 1 {}", day5_1());
    println!("Count 2 {}", day5_2());
}

fn day5_1() -> u32 {
    let file = File::open("./data/05").unwrap();
    let reader = BufReader::new(file);
    reader.lines()
	.map(|s| seat_id(&(s.unwrap())))
	.max()
	.unwrap()
}

fn day5_2() -> u32 {
    let file = File::open("./data/05").unwrap();
    let reader = BufReader::new(file);
    let mut ticket_nos: Vec<u32> = reader.lines()
	.map(|s| seat_id(&(s.unwrap())))
	.collect::<Vec<u32>>();

    ticket_nos.sort_unstable();

    for i in 2..(ticket_nos.len()-1) {
	if ticket_nos[i] - ticket_nos[i-1] == 2 {
	    return ticket_nos[i] - 1;
	}
    }
    0
}

fn seat_id(line: &str) -> u32 {
    let mut row: u32 = 0;
    let mut seat: u32 = 0;
    for (i, c) in line.chars().enumerate() {
	if i < 7  { 	//chars indicating the row
	    if c == 'B' {
		row += 1 << (6-i);
	    }
		} else {  // chars indicating the seat
		    if c == 'R' {
			seat += 1 << (2 - (i - 7));
		    }
		}

    }
    row * 8 + seat
}
// rust-mode... c.f. https://github.com/rust-lang/rust-mode/issues/398
#[test]
		fn test_seat_id() {
		    assert_eq!(seat_id("BFFFBBFRRR"), 567);
		    assert_eq!(seat_id("FFFBBBFRRR"), 119);
		    assert_eq!(seat_id("BBFFBBFRLL"), 820);
}   
		
		#[test]
		fn test_days() {
		    assert_eq!(day5_1(), 970);
		    assert_eq!(day5_2(), 587);
		}
		
