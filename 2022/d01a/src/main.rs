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

	let numbers = input.trim().split('\n')
		.map(|x| x.parse().expect("Not a number"))
		.collect::<Vec<i32>>();

	let mut prev = -1;
	let mut count = -1;

	for n in numbers
	{
		if n > prev
		{
			count = count + 1;
		}
		prev = n;
	}

	println!("{:?}", count);

	Ok(())
}

