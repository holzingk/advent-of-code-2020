use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let passports = init();
    let count = count_valid(&passports);
    println!("Count 1: {}", count);    
}

#[derive(Default, Clone, Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

fn init() -> Vec<Passport> {
    let file = File::open("./data/04").unwrap();
    let reader = BufReader::new(file);
    let mut passports = Vec::new();
    let mut passport = Passport::default();
    for line in reader.lines() {
	let s = line.unwrap();
	parse_into_passport(&s, &mut passport);
	if s.is_empty() {
	    passports.push(passport.clone());
	    passport = Passport::default();
	}
    }
    passports.push(passport);
    passports
}

fn count_valid(passports: &[Passport]) -> u32 {
    passports.iter().fold(0, | acc, passport |
			  {
			      let valid = passport.byr && passport.iyr && passport.eyr && passport.hgt &&
				  passport.hcl && passport.ecl && passport.pid;
			      if valid {
				      acc + 1
				  } else {
				      acc
				  }
			  }
    )
}

fn parse_into_passport(line: &str, passport: &mut Passport){
    let fields_id = line.split_whitespace().map(|s| (&s[..3])).collect::<Vec<_>>();
//    dbg!(&fields_id);
    for field_id in fields_id {
	match field_id {
	    "byr" => passport.byr = true,
	    "iyr" => passport.iyr = true,
	    "eyr" => passport.eyr = true,
	    "hgt" => passport.hgt = true,
	    "hcl" => passport.hcl = true,
	    "ecl" => passport.ecl = true,
	    "pid" => passport.pid = true,
	    "cid" => passport.cid = true,
	    &_ => {},
	}
    }
}

// fn parse_into_passport(line: &str, passport: &mut Passport){
//     let fields_id = line.split_whitespace().map(|s| (&s[..3], &s[4..])).collect::<Vec<_>>();
// //    dbg!(&fields_id);
//     for (field_id, field_value) in fields_id {
// 	dbg!(field_value);
// 	match (field_id, field_value) {
// 	    ("byr", val) => {
// 		let year: u32 = val.parse().unwrap();
// 		if year <= 2002 && year <= 1920 {
// 		    passport.byr = true
// 		}
// 	    },
// 	    ("iyr", val) => passport.iyr = true,
// 	    ("eyr", val) => passport.eyr = true,
// 	    ("hgt", val) => passport.hgt = true,
// 	    ("hcl", val) => passport.hcl = true,
// 	    ("ecl", val) => passport.ecl = true,
// 	    ("pid", val) => passport.pid = true,
// 	    ("cid", val) => passport.cid = true,
// 	    (&_, _) => {},
// 	}
//     }
// }

#[test]
fn star_1() {
    let passports = init();
    let count = count_valid(&passports);
    assert_eq!(count, 239);
}
