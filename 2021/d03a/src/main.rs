use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d03.txt")?;
		file.read_to_string(&mut input)?;
	}

	let mut naughts: [i32; 12] = [0; 12];
	let mut ones: [i32; 12] = [0; 12];
	
	let numbers = input.trim().split('\n')
		.map(|x| i32::from_str_radix(x, 2).unwrap())
		.collect::<Vec<i32>>();

	println!("{:?}", numbers);

	for n in numbers
	{
		for i in 0..12
		{
			let b = 1 << i;
			if ((n & b) == 0)
			{
				naughts[i] = naughts[i] + 1;
			}
			else
			{
				ones[i] = ones[i] + 1;
			}
		}
	}

	let mut gamma = 0;
	let mut epsilon = 0;
	for i in 0..12
	{
		let b = 1 << i;
		if (ones[i] > naughts[i])
		{
			gamma = gamma | b;
		}
		else
		{
			epsilon = epsilon | b;
		}
	}

	println!("{:?}", gamma);
	println!("{:?}", epsilon);
	println!("{:?}", gamma * epsilon);


	Ok(())
}

