use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    println!("Count 1 {}", day15_1(&[20, 0, 1, 11, 6, 3]));
    println!("Count 1 {}", day15_2(&[20, 0, 1, 11, 6, 3]));

} 

fn day15_1(input: &[u64]) -> u64 {
    nth(&input, 2020)
}

fn day15_2(input: &[u64]) -> u64 {
    nth(&input, 30000000)
}

fn nth(input: &[u64], n: u64) -> u64 {
    let mut numbers: HashMap<u64, Vec<u64>> = HashMap::new();
    numbers.insert(0, Vec::new());
    for (i, e) in input.iter().enumerate() {
	numbers.insert(e.to_owned(), vec![i as u64 + 1]);
    }
    let mut last_number = input[input.len() - 1].to_owned();
    for i in input.len() + 1 .. n as usize + 1 {

	// last time first observed
	if numbers.get(&last_number).unwrap().len() < 2 {	    
	    if numbers.get(&0).unwrap().len() == 2 {
		numbers.get_mut(&0).unwrap().rotate_left(1);
		numbers.get_mut(&0).unwrap()[1] = i as u64;
	    } else {
		numbers.get_mut(&0).unwrap().push(i as u64);
	    }
	    last_number = 0;
	} else {
	    let last_time = numbers.get(&last_number).unwrap()[0].to_owned();
	    let new_number = i as u64 - 1 - last_time;
	    match numbers.entry(new_number) {
		// with capacity...
	     	Vacant(entry) => { entry.insert(vec![i as u64]); },
	     	Occupied(mut entry) => {
		    if entry.get().len() > 1 {
			entry.get_mut().rotate_left(1);
			entry.get_mut()[1] = i as u64;
		    } else {
			entry.get_mut().push(i as u64);
		    }
		},
	    }
	    last_number = new_number;
	}
    }
    last_number
}

#[test]
fn assignment1() {
    assert_eq!(day15_1(&[0, 3, 6]), 436);
    assert_eq!(day15_1(&[1, 3, 2]), 1);
    assert_eq!(day15_1(&[2, 1, 3]), 10);
    assert_eq!(day15_1(&[1, 2, 3]), 27);
    assert_eq!(day15_1(&[2, 3, 1]), 78);
    assert_eq!(day15_1(&[3, 2, 1]), 438);
    assert_eq!(day15_1(&[3, 1, 2]), 1836);
}

 #[test]
 fn assignment2() {
     assert_eq!(day15_2(&[20,0,1,11,6,3]), 436);
}
