use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

struct Id
{
	byr: String,
	iyr: String,
	eyr: String,
	hgt: String,
	hcl: String,
	ecl: String,
	pid: String,
	cid: String,
}

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

	let mut count = 0;
	let mut byr = false;
	let mut iyr = false;
	let mut eyr = false;
	let mut hgt = false;
	let mut hcl = false;
	let mut ecl = false;
	let mut pid = false;
	let mut cid = false;
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
			cid = false;
		}
		else
		{
			if p.starts_with("byr") { byr = true; }
			if p.starts_with("iyr") { iyr = true; }
			if p.starts_with("eyr") { eyr = true; }
			if p.starts_with("hgt") { hgt = true; }
			if p.starts_with("hcl") { hcl = true; }
			if p.starts_with("ecl") { ecl = true; }
			if p.starts_with("pid") { pid = true; }
			if p.starts_with("cid") { cid = true; }
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

