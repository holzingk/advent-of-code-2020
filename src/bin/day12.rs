use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    North,
    South,
    East,
    West,
    Forward,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction {
    op: Op,
    arg: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct ShipState {
    direction: Direction,
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Waypoint {
//    direction: Direction,
    x: i32,
    y: i32,
}

impl ShipState {
    fn forward(&mut self, arg: i32) {
	match self.direction {
	    Direction::North => self.y += arg,
	    Direction::South => self.y -= arg,
	    Direction::East => self.x += arg,
	    Direction::West => self.x -= arg,
	}
    }

    fn rotate_right(&mut self, deg: i32) {
	self.rotate(-deg);
    }

    fn rotate_left(&mut self, deg: i32) {
	self.rotate(deg);
    }

    fn rotate(&mut self, mut deg: i32) {
	let orientation = match self.direction {
	    Direction::North => 0,
	    Direction::South => 180,
	    Direction::East => 270,
	    Direction::West => 90,
	};
	if deg < 0 { deg += 360; }
	let new_orientation = (orientation + deg) % 360;
	self.direction = match new_orientation {
	    0 => Direction::North,
	    180 => Direction::South,
	    270 => Direction::East,
	    90 => Direction::West,
	    _ => panic!("catastrophic failure"),
	};	    
    }    
}

impl Waypoint {

    fn rotate_right(&mut self, deg: i32) {
	match deg {
	    90 => {
		let (old_x, old_y) = (self.x, self.y);
		self.x = old_y;
		self.y = -old_x
	    },
	    180 => {
		let (old_x, old_y) = (self.x, self.y);
		self.x = -old_x;
		self.y = -old_y;
	    },
	    270 => {
		let (old_x, old_y) = (self.x, self.y);
		self.x = -old_y;
		self.y = old_x
	    }
	    _ => panic!("o.O"),
	}
    }

    fn rotate_left(&mut self, deg: i32) {
	self.rotate_right(360-deg);
    }
}

fn main() {
    println!("Count 1 {}", day12_1("data/12"));
    println!("Count 2 {}", day12_2("data/12"));
}

fn day12_1(file_name: &str) -> u64 {
    let instructions = parse(file_name);
    let mut state = ShipState { direction: Direction::East, x: 0, y: 0 };
    for instruction in instructions {
	match instruction.op {
	    // translative
	    Op::North => state.y += instruction.arg,
	    Op::South => state.y -= instruction.arg,
	    Op::East => state.x += instruction.arg,
	    Op::West => state.x -= instruction.arg,
	    Op::Forward => state.forward(instruction.arg),
	    // rotative
	    Op::Right => state.rotate_right(instruction.arg),
	    Op::Left => state.rotate_left(instruction.arg),
	}
	//dbg!(&instruction, &state.x, &state.y, &state.direction);
    }
    (state.x.abs() + state.y.abs()) as u64
}

fn day12_2(file_name: &str) -> u64 {
    let instructions = parse(file_name);
    let mut ship = ShipState { direction: Direction::East, x: 0, y: 0 };
    let mut waypoint = Waypoint { x: 10, y: 1};
    for instruction in instructions {
	match instruction.op {
	    // Waypoint
	    Op::North => waypoint.y += instruction.arg,
	    Op::South => waypoint.y -= instruction.arg,
	    Op::East => waypoint.x += instruction.arg,
	    Op::West => waypoint.x -= instruction.arg,
	    // Translative
	    Op::Forward => {
		ship.x += waypoint.x * instruction.arg;
		ship.y += waypoint.y * instruction.arg;
	    },
	    // rotative
	    Op::Right => waypoint.rotate_right(instruction.arg),
	    Op::Left => waypoint.rotate_left(instruction.arg),
	}
	//dbg!(&instruction, &waypoint, &ship);
    }
    (ship.x.abs() + ship.y.abs()) as u64
}

fn parse(file_name: &str) -> Vec<Instruction> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
	.map(|l| {
	    let s = l.unwrap();
	    let op = match s.chars().next().unwrap() {
		'N' => Op::North,
		'S' => Op::South,
		'E' => Op::East,
		'W' => Op::West,
		'F' => Op::Forward,
		'R' => Op::Right,
		'L' => Op::Left,
		_ => panic!("parse error"),
	    };
	    let arg = s[1..].parse().unwrap();
	    Instruction {
		op,
		arg,
	    }
	})
	.collect()
}

#[test]
fn parse_test() {
    let instructions = parse("data/12_sample");
    assert_eq!(instructions[0], Instruction{ op: Op::Forward, arg: 10 });
}

#[test]
fn assignment_1() {
    assert_eq!(day12_1("data/12_sample"), 25);
    assert_eq!(day12_1("data/12"), 1457);
}

#[test]
fn assignment_2() {
    assert_eq!(day12_2("data/12_sample"), 286);
    assert_eq!(day12_2("data/12"), 106860);
}

