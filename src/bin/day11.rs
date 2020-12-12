use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Place {
    Floor,
    EmptySeat,
    OccupiedSeat
}

type WaitingHall = Vec<Vec<Place>>;

trait WaitingHallMethods {
    fn pad(&mut self);
    fn occupied(&self) -> u32;
    fn apply_rules1(&mut self) -> u32;
    fn apply_rules2(&mut self) -> u32;
    fn check_direction(&self, i: i32, j: i32,
		       x: i32, y: i32) -> Place;
    fn print(&self);
}


impl WaitingHallMethods for WaitingHall {
    
    fn print(&self) {
	let line_length = self[0].len();
    	for line in self {
	    assert_eq!(line.len(), line_length);
	    for c in line {
		match c {
		    Place::Floor => print!("."),
		    Place::EmptySeat => print!("L"),
		    Place::OccupiedSeat => print!("#"),
		}
	    }
	    println!();
	}
	println!();
	println!();
    }

    fn pad(&mut self) {
	// put row in top
	self.insert(0, iter::repeat(Place::Floor).take(self[0].len()).collect());
	// put row at bottom
	self.push(iter::repeat(Place::Floor).take(self[0].len()).collect());
	let hall_len = self.len();
	// insert columns left and right
	for row in self.iter_mut().take(hall_len) {	    
	    row.insert(0, Place::Floor);
	    row.push(Place::Floor);
	}
    }

    fn apply_rules1(&mut self) -> u32 {
	let mut to_occupy = Vec::new();
	let mut to_empty = Vec::new();
	let num_rows = self.len();
	let num_columns = self[0].len();
	for (i, row) in self.iter().enumerate().skip(1).take(num_rows - 2) {
	    for (j, _place) in row.iter().enumerate().skip(1).take(num_columns - 2) {
		let surroundings = [
		    self[i-1][j-1], // top left
		    self[i][j-1],  // left
		    self[i+1][j-1],
		    self[i+1][j], // down
		    self[i+1][j+1],
		    self[i][j+1], // right
		    self[i-1][j+1],
		    self[i-1][j] // top
		];
		if
		    self[i][j] == Place::EmptySeat &&
		    surroundings.iter().copied()
		    .all(|e| e == Place::Floor || e == Place::EmptySeat)
		{
		    to_occupy.push((i, j));
		}
		else if
		    self[i][j] == Place::OccupiedSeat &&
		    surroundings.iter().copied()
		    .filter(|e| *e == Place::OccupiedSeat)
		    .count() >= 4
		{
		    to_empty.push((i, j));
		}
	    }
	}
	for (i, j) in &to_occupy {
	    self[*i][*j] = Place::OccupiedSeat;
	}
	for (i, j) in &to_empty {
	    self[*i][*j] = Place::EmptySeat;
	}
	(to_occupy.len() + to_empty.len()) as u32
    }

    fn apply_rules2(&mut self) -> u32 {
	let mut to_occupy = Vec::new();
	let mut to_empty = Vec::new();
	let num_rows = self.len();
	let num_columns = self[0].len();
	for (i, row) in self.iter().enumerate().skip(1).take(num_rows - 2) {
	    for (j, _place) in row.iter().enumerate().skip(1).take(num_columns - 2) {
		let directions = [(-1,-1), (0,-1), (1,-1), (1,0),
				  (1,1), (0,1), (-1,1), (-1,0)];
		let mut visible_occupied = 0;
		for (x, y) in directions.iter() {
		    match self.check_direction(i as i32, j as i32, *x, *y) {
			Place::OccupiedSeat => visible_occupied += 1,
			Place::EmptySeat => {},
			Place::Floor => {},
		    }
		}
	//	dbg!(&i, &j, &visible_occupied, &visible_empty);
		if self[i][j] == Place::EmptySeat &&
		    visible_occupied == 0
		{
		    to_occupy.push((i, j));
		}
		else if self[i][j] == Place::OccupiedSeat &&
		    visible_occupied >= 5
		{
		    to_empty.push((i, j));
		}
	    }
	}
	for (i, j) in &to_occupy {
	    self[*i][*j] = Place::OccupiedSeat;
	}
	for (i, j) in &to_empty {
	    self[*i][*j] = Place::EmptySeat;
	}
	(to_occupy.len() + to_empty.len()) as u32
    }
    
    fn check_direction(&self,
		       mut i: i32, mut j: i32,
		       x: i32, y: i32) -> Place {
	i += x;
	j += y;
	while i > 0 && j > 0 &&
	    i < self.len() as i32 && j < self[i as usize].len() as i32 {
		if self[i as usize][j as usize] == Place::OccupiedSeat {
		    return Place::OccupiedSeat
		} else if self[i as usize][j as usize] == Place::EmptySeat {
		    return Place::EmptySeat
		}
		i += x;
		j += y;

	    }
	Place::Floor
    }
    
    fn occupied(&self) -> u32 {
	let mut occupied = 0;
	for (i, row) in self.iter().enumerate() {
	    let mut occupied_row = 0;
	    for (j, _place) in row.iter().enumerate() {
		if self[i][j] == Place::OccupiedSeat {
		    occupied += 1;
		    occupied_row += 1;
		}
	    }
	    println!(" {}", occupied_row);
	}
	occupied
    }
}

fn main() {
    println!("Count 1 {}", day11_1("data/11"));
    println!("Count 2 {}", day11_2("data/11"));
}

fn day11_1(file_name: &str) -> u64 {
    let mut wh = parse(file_name);
    let mut changes = 1;
    while changes > 0 {
	changes = wh.apply_rules1();
	println!("Changes: {}", changes);
	wh.print();
	println!("Occupied: {}", wh.occupied());
	println!();
    }
    println!();
    wh.occupied().into()
}

fn day11_2(file_name: &str) -> u64 {
    let mut wh = parse(file_name);
    let mut changes = 1;
    while changes > 0 {
	changes = wh.apply_rules2();
	println!("Changes: {}", changes);
	wh.print();
	println!("Occupied: {}", wh.occupied());
	println!();
    }
    println!();
    wh.occupied().into()
}


fn parse(file_name: &str) -> WaitingHall {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut wh: WaitingHall = reader.lines()
	.map(|l| l.unwrap().chars() 
	     .map(|c| match c {
		 '.' => Place::Floor,
		 'L' => Place::EmptySeat,
		 '#' => Place::OccupiedSeat,
		 _ => panic!("parsing error"),
	     }
	     )
	     .collect()
	     )
	.collect();
    wh.pad();
    wh
}

#[test]
fn hallway_test() {
    let parsed = parse("data/11_sample");
    assert_eq!(parsed[1][1], Place::EmptySeat);
    assert_eq!(day11_1("data/11_sample"), 37);
    assert_eq!(day11_1("data/11"), 2178);
    
    assert_eq!(day11_2("data/11_sample"), 26);
    assert_eq!(day11_2("data/11"), 1978);
}
