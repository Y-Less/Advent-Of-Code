use std::fs::File;
use std::io::prelude::*;

fn bitmap(s: &str) -> (u64)
{
	let mut ret: u64 = 0;
	let len = s.len();
	for i in 0 .. len
	{
		{
			// Apparently `nth` removes the data...
			let j = s.bytes().nth(i);
			let b = j.unwrap() - 0x41; // - 'A'
			if b >= 26
			{
				ret = ret | (1u64 << b - 32);
			}
			else
			{
				ret = ret | (1u64 << b + 26);
			}
		}
	}
	ret
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d04.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	let mut total = 0;
	for line in lines
	{
		let pairs = line.trim().split(',').collect::<Vec<&str>>();
		let l = pairs[0].split('-')
			.map(|x| x.parse().expect("Not a number"))
			.collect::<Vec<i32>>();
		let r = pairs[1].split('-')
			.map(|x| x.parse().expect("Not a number"))
			.collect::<Vec<i32>>();
		if l[0] <= r[0] && l[1] >= r[1]
		{
			//println!("{:?} is fully contained in {:?}", r, l);
			total = total + 1;
		}
		else if r[0] <= l[0] && r[1] >= l[1]
		{
			//println!("{:?} is fully contained in {:?}", r, l);
			total = total + 1;
		}
	}
	
	println!("{:?}", total);

	Ok(())
}

