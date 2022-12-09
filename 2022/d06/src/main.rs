use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d06.txt")?;
		file.read_to_string(&mut input)?;
	}

	for ch in input
	{
		println!("{:?}", ch);
	}

	Ok(())
}

