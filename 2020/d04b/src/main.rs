use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d04.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let list = Regex::new(r"\s").unwrap().split(&input)
		//.map(|s| s.as_bytes())
		.collect::<Vec<&str>>();

	let byr_regex = Regex::new(r"\b(19[2-9][0-9]|2000|2001|2002)\b").unwrap();
	let iyr_regex = Regex::new(r"\b(201[0-9]|2020)\b").unwrap();
	let eyr_regex = Regex::new(r"\b(202[0-9]|2030)\b").unwrap();
	let hgt_regex = Regex::new(r"\b((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)\b").unwrap();
	let hcl_regex = Regex::new(r"(#[0-9a-f]{6})\b").unwrap();
	let ecl_regex = Regex::new(r"\b(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
	let pid_regex = Regex::new(r"\b([0-9]{9})\b").unwrap();
	//let cidRegex = Regex::new(r"").unwrap();

	let mut count = 0;
	let mut byr = false;
	let mut iyr = false;
	let mut eyr = false;
	let mut hgt = false;
	let mut hcl = false;
	let mut ecl = false;
	let mut pid = false;
	//let mut cid = false;
	for p in list
	{
		if p == ""
		{
			if byr && iyr && eyr && hgt && hcl && ecl && pid
			{
				count = count + 1;
			}
			byr = false;
			iyr = false;
			eyr = false;
			hgt = false;
			hcl = false;
			ecl = false;
			pid = false;
			//cid = false;
		}
		else
		{
			let value = p.split(":").last().unwrap();
			if p.starts_with("byr") && byr_regex.is_match(value) { byr = true; }
			if p.starts_with("iyr") && iyr_regex.is_match(value) { iyr = true; }
			if p.starts_with("eyr") && eyr_regex.is_match(value) { eyr = true; }
			if p.starts_with("hgt") && hgt_regex.is_match(value) { hgt = true; }
			if p.starts_with("hcl") && hcl_regex.is_match(value) { hcl = true; }
			if p.starts_with("ecl") && ecl_regex.is_match(value) { ecl = true; }
			if p.starts_with("pid") && pid_regex.is_match(value) { pid = true; }
			//if p.starts_with("cid") { cid = true; }
		}
	}


	println!("{:?}", count);

	//let a = step(1, 1, &field);
	//let b = step(3, 1, &field);
	//let c = step(5, 1, &field);
	//let d = step(7, 1, &field);
	//let e = step(1, 2, &field);
    //
	//
	//println!("{:?}", a);
	//println!("{:?}", b);
	//println!("{:?}", c);
	//println!("{:?}", d);
	//println!("{:?}", e);
	//println!("{:?}", a * b * c * d * e);
	
	Ok(())
}

