use std::fs::File;
use std::io::prelude::*;

//#[warn(non_snake_case, none)]

fn deBruijn(data: u64) -> u64
{
	// Couldn't get this to work in rust...
	/*
	let scDeBruijn = //: [u64, 64] =
		[
			1,  2,  3, 54,  4,  8, 55, 28,  5, 39, 42,  9, 35, 56, 49, 29,
		   63,  6, 40, 47, 45, 43, 23, 10, 25, 36, 60, 57, 50, 19, 30, 12,
		   64, 53,  7, 27, 38, 41, 34, 48, 62, 46, 44, 22, 24, 59, 18, 11,
		   52, 26, 37, 33, 61, 21, 58, 17, 51, 32, 20, 16, 31, 15, 14, 13
		];
	scDeBruijn[(((data & (!data + 1)) * 0x022FDD63CC95386D) >> 27) as usize]
	*/
	let mut bit = 0;
	for i in 0 .. 64
	{
		if data & (1 << i) != 0
		{
			bit = i + 1;
		}
	}
	bit
}

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
				//println!("{:?}, {:?}", b, b - 32);
				//let a = j.unwrap() - 0x61; // - 'a'
				ret1 = ret1 | (1u64 << b - 32);
			}
			else
			{
				//println!("{:?}, {:?}", b, b + 26);
				ret1 = ret1 | (1u64 << b + 26);
			}
		}
		//println!("{:?}", bytes);
		{
			let j = s.bytes().nth(len - i - 1);
			let b = j.unwrap() - 0x41; // - 'A'
			if b >= 26
			{
				//println!("{:?}, {:?}", b, b - 32);
				//let a = j.unwrap() - 0x61; // - 'a'
				ret2 = ret2 | (1u64 << b - 32);
			}
			else
			{
				//println!("{:?}, {:?}", b, b + 26);
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
		let mut file = File::open("../inputs/d03.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	let mut total = 0;
	for line in lines
	{
		let b = bitmap(line);
		total = total + deBruijn(b.2);
		
	}
	
	println!("{:?}", total);

	Ok(())
}

