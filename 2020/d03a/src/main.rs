use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d03.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let field = input.trim().split('\n')
		.collect::<Vec<&str>>();


	let mut x = 0;
	let mut count = 0;
	for p in field
	{
		println!("{:?}", p);
		let line = p.as_bytes();
		let y = line[x % line.len()];
		x = x + 3;
		if y == 35
		{
			count = count + 1;
		}
	}
	println!("{:?}", count);


	Ok(())
}

