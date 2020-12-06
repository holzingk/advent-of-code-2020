use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    println!("Count 1 {}", day6_1());
    println!("Count 2 {}", day6_2());
}

#[derive(Default, Clone)]
struct Group {
    pub members: Vec<HashSet<char>>,
}

impl Group {
    fn count_anyone(&self) -> u32 {
	let anyone = self.members
	    .iter()
	    .fold(
		HashSet::new(),
		| acc, member | {
		    member
			.union(&acc)
			.cloned()
			.collect()
		});
	anyone.len() as u32    
    }

    fn add(&mut self, hs: HashSet<char>) {
	self.members.push(hs);
    }
	
    fn count_all(&self) -> u32 {
	let all = self.members
	    .iter()
	    .fold(
		None,
		| acc, member | {
		    match acc {
			None => {
			    Some(member.clone())
			},
			Some(hs) => {
			    Some(member
				 .intersection(&hs)
				 .cloned()
				 .collect::<HashSet<_>>()
			    )
			}
		    }
		});
	all.unwrap().len() as u32
    }

}

fn day6_1() -> u32 {
    let groups = parse();
    groups.iter().fold(0, |acc, g| acc + g.count_anyone())
}

fn day6_2() -> u32 {
    let groups = parse();
    groups.iter().fold(0, |acc, g| acc + g.count_all())
}

fn parse() -> Vec<Group> {
    let file = File::open("./data/06").unwrap();
    let reader = BufReader::new(file);
    let mut groups = Vec::new();
    let mut group = Group::default();
    for line in reader.lines() {
	let s = line.unwrap();
	if s.is_empty() { // current group is done
	    groups.push(group.clone());
	    group = Group::default();
	} else {
	    let line_set = parse_into_lineset(&s);
	    group.add(line_set);
	}
    }
    groups.push(group);
    groups
}

fn parse_into_lineset(line: &str) -> HashSet<char> {
    let mut line_set = HashSet::new();
    for c in line.chars() {
	line_set.insert(c);
    }
    line_set
}

#[test]
fn lineset_test() {
    let hs = parse_into_lineset("yryn");
    assert!(hs.contains(&'y'));
    assert!(hs.contains(&'r'));
    assert!(hs.contains(&'n'));	
}


#[test]
fn cookies() {
    assert_eq!(day6_1(), 6457);
    assert_eq!(day6_2(), 3260);	
}
