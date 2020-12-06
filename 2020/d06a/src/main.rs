use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d06.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let list = input.split("\n\n")
		.collect::<Vec<&str>>();

	let mut count = 0;
	for p in list
	{
		let mut arr: [bool; 26] = [false; 26];
		// Get the number of unique letters in this set.  Ignore new lines.
		for c in p.bytes()
		{
			if c != 10
			{
				//println!("{:?}", c);
				if !arr[(c - 0x61) as usize]
				{
					count = count + 1;
				}
				arr[(c - 0x61) as usize] = true;
			}
		}
		println!("{:?}", arr);
	}
	println!("{:?}", count);

	Ok(())
}

