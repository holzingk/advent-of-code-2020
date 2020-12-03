use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Index;

fn main() {
    println!("Count 1: {}", cookie3_1());
    println!("Count 2: {}", cookie3_2());
    
}

struct MapRow {
    row: Vec<bool>,
}

impl Index<usize> for MapRow {
    type Output = bool;

    fn index(&self, i: usize) -> &Self::Output {
	&self.row[i % self.row.len()]
    }
}

fn init() -> Vec<MapRow> {
    let file = File::open("./data/03").unwrap();
    let reader = BufReader::new(file);
    reader.lines()
	.map(|s| parse_line(&(s.unwrap()))).collect()
}

fn cookie3_1() -> u32 {
    trees(3, 1)
}

fn cookie3_2() -> u32 {
    trees(1, 1) * trees(3, 1) * trees(5, 1) * trees(7, 1) * trees(1, 2)
}

fn trees(right: u16, down: u16) -> u32 {
    let map = init();
    let (_, trees_tot) = map.iter()
	.step_by(down as usize)
	.fold(
	    (0, 0),
	    |(offset, trees), l| {
		let tree = l[offset];
		let new_offset = offset + right as usize;
		if tree {
		    (new_offset, trees + 1)
		} else {
		    (new_offset, trees)
		}
	    }
	);
    trees_tot
}

    
fn parse_line(line: &str) -> MapRow {
    MapRow { row:
	     line.chars()
	     .map(|c|
		  if c == '.' { false }
		  else if c == '#' { true }
		  else { panic!("Parsing error") })
	     .collect()
    }
}
	
#[test]
fn all_lines_equal_length() {
    let map = init();
    let len_first_row = map[0].row.len();
    assert!(map.iter().all(|r| r.row.len() == len_first_row));
}
#[test]
fn test_index() {
    let map = init();
    assert!(map[0].row[0] == false);
    assert!(map[0][0] == false);
    assert!(map[0][12] == true);
    let len_first_row = map[0].row.len();
    assert!(map[0][len_first_row] == false);
    assert!(map[0][len_first_row + 12] == true);	
}

#[test]
fn first_cookie() {
    assert_eq!(cookie3_1(), 171);
}
#[test]
fn second_cookie() {
    assert_eq!(cookie3_2(), 1206576000);
}
