use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}

	let mut idx: usize;
	let mut idx = 0;
	let mut steps: [char; 14];
	steps = [ ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' ];
	idx = 0;
	for ch in input.chars()
	{
		idx = idx + 1;
		for i in 0..13
		{
			steps[i] = steps[i + 1];
		}
		steps[13] = ch;
		if idx >= 4
		{
			let mut diff = true;
			for i in (14 - 4)..13
			{
				for j in i+1..14
				{
					if steps[i] == steps[j]
					{
						diff = false;
						break;
					}
				}
				if !diff
				{
					break;
				}
			}
			if diff
			{
				println!("Part A: {:?}", idx);
				break;
			}
		}
	}
	steps = [ ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' ];
	idx = 0;
	for ch in input.chars()
	{
		idx = idx + 1;
		for i in 0..13
		{
			steps[i] = steps[i + 1];
		}
		steps[13] = ch;
		if idx >= 14
		{
			let mut diff = true;
			for i in (14 - 14)..13
			{
				for j in i+1..14
				{
					if steps[i] == steps[j]
					{
						diff = false;
						break;
					}
				}
				if !diff
				{
					break;
				}
			}
			if diff
			{
				println!("Part B: {:?}", idx);
				break;
			}
		}
	}

	Ok(())
}

