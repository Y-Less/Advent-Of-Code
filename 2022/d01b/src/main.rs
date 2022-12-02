use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d01.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	let mut highest1: i32 = 0;
	let mut highest2: i32 = 0;
	let mut highest3: i32 = 0;
	let mut accum: i32 = 0;
	for line in lines
	{
		if line.len() == 0
		{
			if accum > highest1
			{
				highest3 = highest2;
				highest2 = highest1;
				highest1 = accum;
			}
			else if accum > highest2
			{
				highest3 = highest2;
				highest2 = accum;
			}
			else if accum > highest3
			{
				highest3 = accum;
			}
			accum = 0;
		}
		else
		{
			let n: i32 = line.parse().unwrap();
			accum = accum + n;
		}
	}
	if accum > highest1
	{
		highest3 = highest2;
		highest2 = highest1;
		highest1 = accum;
	}
	else if accum > highest2
	{
		highest3 = highest2;
		highest2 = accum;
	}
	else if accum > highest3
	{
		highest3 = accum;
	}

	let highest = highest1 + highest2 + highest3;
	println!("{:?}", highest);

	Ok(())
}

