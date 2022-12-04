use std::fs::File;
use std::io::prelude::*;

fn bitmap(s: &str) -> (u64, u64, u64)
{
	let mut ret1: u64 = 0;
	let mut ret2: u64 = 0;
	//let mut bytes = s.bytes();
	let len = s.len();
	for i in 0 .. (len / 2)
	{
		//println!("{:?}, {:?}, {:?}", s, i, len - i - 1);
		//println!("{:?}", s[0..1]);
		{
			// Apparently `nth` removes the data...
			let j = s.bytes().nth(i);
			let b = j.unwrap() - 0x41; // - 'A'
			//println!("{:?}, {:?}", a, b);
			if b >= 26
			{
				println!("{:?}, {:?}", b, b - 32);
				//let a = j.unwrap() - 0x61; // - 'a'
				ret1 = ret1 | (1u64 << b - 32);
			}
			else
			{
				println!("{:?}, {:?}", b, b + 26);
				ret1 = ret1 | (1u64 << b + 26);
			}
		}
		//println!("{:?}", bytes);
		{
			let j = s.bytes().nth(len - i - 1);
			let b = j.unwrap() - 0x41; // - 'A'
			if b >= 26
			{
				println!("{:?}, {:?}", b, b - 32);
				//let a = j.unwrap() - 0x61; // - 'a'
				ret2 = ret2 | (1u64 << b - 32);
			}
			else
			{
				println!("{:?}, {:?}", b, b + 26);
				ret2 = ret2 | (1u64 << b + 26);
			}
		}
	}
	(ret1, ret2, ret1 & ret2)
}

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d02.txt")?;
		file.read_to_string(&mut input)?;
	}

	/*let lines = input.trim().split('\n');
	let mut total = 0;
	for line in lines
	{
		let original = line.trim().split(' ').collect::<Vec<&str>>();
		//println!("{:?}", original[0]);
		total = total + score(original[0], original[1]);
		println!("{:?}", score(original[0], original[1]));
	}
	println!("{:?}", total);*/
	
	let s = "Helloe";
	println!("{:?}", s.bytes().nth(2));
	println!("{:?}", s.bytes().nth(3));
	println!("{:?}", bitmap(s));

	Ok(())
}

