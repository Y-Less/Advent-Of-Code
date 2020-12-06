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
		let mut arr: [i32; 26] = [0; 26];
		// Get the number of unique letters in this set.  Ignore new lines.
		let mut members = 1;
		for c in p.bytes()
		{
			if c == 10
			{
				members += 1;
			}
			else
			{
				arr[(c - 0x61) as usize] += 1;
			}
		}
		for i in 0..26
		{
			if arr[i] == members
			{
				count += 1;
			}
		}
		println!("{:?}", members);
	}
	println!("{:?}", count);

	Ok(())
}

