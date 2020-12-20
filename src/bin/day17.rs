use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Coordinate {
    Active,
    Inactive,
}

fn main() {
    println!("Count 1 {}", day17_1("data/17"));
    println!("Count 2 {}", day17_2("data/17"));
} 

const DIM: usize = 28;

#[allow(clippy::needless_range_loop)]
fn day17_1(file_name: &str) -> u64 {
    let mut space = parse(&file_name);
    let x_dim = space.len();
    let y_dim = space[0].len();
    let z_dim = space[0][0].len();
    for _i in 0..6 {
	let mut to_activate = Vec::new();
	let mut to_disable = Vec::new();
	for x in 1..x_dim - 1 {
	    for y in 1..y_dim - 1 {
		for z in 1..z_dim - 1 {
		    let neighbors = get_neighbors(x, y, z);
		    let active_neighbors: u32 = neighbors
			.iter()
			.fold(0,
			      |acc, neighbor|
			      if space[neighbor.0 as usize][neighbor.1 as usize][neighbor.2 as usize]
			      == Coordinate::Active {
				  acc + 1
			      } else {
				  acc
			      });
		    if space[x][y][z] == Coordinate::Active && !(active_neighbors == 2 || active_neighbors == 3) {
			to_disable.push((x, y, z));
		    } else if space[x][y][z] == Coordinate::Inactive && active_neighbors == 3 {
			to_activate.push((x, y, z));
		    }
		}
	    }
	}
	for (x, y, z) in &to_disable {
	    space[*x][*y][*z] = Coordinate::Inactive;
	}
	for (x, y, z) in &to_activate {
	    space[*x][*y][*z] = Coordinate::Active;
	}
    }
    print(&space);
    let mut active_count = 0;
    for x in 0..DIM {
	for y in 0..DIM {
	    for z in 0..DIM {
		if space[x][y][z] == Coordinate::Active {
		    active_count += 1;
		}
	    }
	}
    }
    active_count
}

#[allow(clippy::needless_range_loop)]
fn print(space: &[Vec<Vec<Coordinate>>]) {
    for z in 0..DIM {
	println!("z: {}", z);
	for y in 0..DIM {
	    for x in 0..DIM {
		match space[x][y][z] {
		    Coordinate::Active => print!("#"),
		    Coordinate::Inactive => print!("."),
		}
	    }
	    println!();
	}
	println!();
    }
}

#[allow(clippy::needless_range_loop)]
fn day17_2(file_name: &str) -> u64 {
    let mut space = parse2(&file_name);
    let x_dim = space.len();
    let y_dim = space[0].len();
    let z_dim = space[0][0].len();
    let w_dim = space[0][0][0].len();
    for _i in 0..6 {
	let mut to_activate = Vec::new();
	let mut to_disable = Vec::new();
	for x in 1..x_dim - 1 {
	    for y in 1..y_dim - 1 {
		for z in 1..z_dim - 1 {
		    for w in 1..w_dim - 1 {
			let neighbors = get_neighbors2(x, y, z, w);
			let active_neighbors: u32 = neighbors
			    .iter()
			    .fold(0,
				  |acc, neighbor|
				  if space[neighbor.0][neighbor.1][neighbor.2][neighbor.3]
				  == Coordinate::Active {
				      acc + 1
				  } else {
				      acc
				  });
			if space[x][y][z][w] == Coordinate::Active &&
			    !(active_neighbors == 2 || active_neighbors == 3) {
				to_disable.push((x, y, z, w));
			    } else if space[x][y][z][w] == Coordinate::Inactive &&
			    active_neighbors == 3 {
				to_activate.push((x, y, z, w));
			    }
		    }
		}
	    }
	}
	for (x, y, z, w) in &to_disable {
	    space[*x][*y][*z][*w] = Coordinate::Inactive;
	}
	for (x, y, z, w) in &to_activate {
	    space[*x][*y][*z][*w] = Coordinate::Active;
	}
    }
    let mut active_count = 0;
    for x in 0..DIM {
	for y in 0..DIM {
	    for z in 0..DIM {
		for w in 0..DIM {
		    if space[x][y][z][w] == Coordinate::Active {
			active_count += 1;
		    }
		}
	    }
	}
    }
    active_count
}

fn get_neighbors(x: usize, y: usize, z: usize)
		 -> Vec<(usize, usize, usize)> {
    let mut neighbors = Vec::new();
    for i in -1..2 {
	for j in -1..2 {
	    for k in -1..2 {
		if (i, j, k) == (0, 0, 0) { continue }
		neighbors.push((
		    (x as i32 + i) as usize,
		    (y as i32 + j) as usize,
		    (z as i32 + k) as usize
		));
	    }
	}
    }
    assert_eq!(neighbors.len(), 26);
    neighbors
}

fn get_neighbors2(x: usize, y: usize, z: usize, w: usize)
		 -> Vec<(usize, usize, usize, usize)> {
    let mut neighbors = Vec::new();
    for i in -1..2 {
	for j in -1..2 {
	    for k in -1..2 {
		for l in -1..2 {
		    if (i, j, k, l) == (0, 0, 0, 0) { continue }
		neighbors.push((
		    (x as i32 + i) as usize,
		    (y as i32 + j) as usize,
		    (z as i32 + k) as usize,
		    (w as i32 + l) as usize
		));
		}
	    }
	}
    }
    assert_eq!(neighbors.len(), 80);
    neighbors
}


fn parse(file_name: &str) -> Vec<Vec<Vec<Coordinate>>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut ret = vec![vec![vec![Coordinate::Inactive; DIM]; DIM]; DIM];
    for (y, line) in reader.lines().enumerate() {
	let s = line.unwrap();	
	for (x, c) in s.chars().enumerate() {
	    ret[x + DIM/2][y + DIM/2][DIM/2] = match c {
		'#' => Coordinate::Active,
		'.' => Coordinate::Inactive,
		_ => panic!("parse error"),
	    };
	}
    }
    ret
}

fn parse2(file_name: &str) -> Vec<Vec<Vec<Vec<Coordinate>>>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut ret = vec![vec![vec![vec![Coordinate::Inactive; DIM]; DIM]; DIM]; DIM];
    for (y, line) in reader.lines().enumerate() {
	let s = line.unwrap();	
	for (x, c) in s.chars().enumerate() {
	    ret[x + DIM/2][y + DIM/2][DIM/2][DIM/2] = match c {
		'#' => Coordinate::Active,
		'.' => Coordinate::Inactive,
		_ => panic!("parse error"),
	    };
	}
    }
    ret
}

#[test]
fn assignment_1_sample() {
    assert_eq!(day17_1("data/17_sample"), 112);

}
#[test]
fn assignment_1_real() {
    assert_eq!(day17_1("data/17"), 247);
}

#[test]
fn assignment_2_sample() {
    assert_eq!(day17_2("data/17_sample"), 848);
}


#[test]
fn assignment_2_real() {
    assert_eq!(day17_2("data/17"), 1392);
}
