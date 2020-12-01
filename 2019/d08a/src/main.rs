use std::fs::File;
use std::io::prelude::*;

const SIZE: i32 = 25 * 6;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d08.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	//let image = input.trim().split(',')
	//	.map(|x| x.parse().expect("Not a number"))
	//	.collect::<Vec<i32>>();
	
	let mut min = SIZE;
	let mut ans = 0;

	let mut counts = [0, 0, 0];

	let mut i = 0;

	for p in input.bytes()
	{
		//println!("{}", p);
		counts[(p - 48) as usize] += 1;
		i += 1;

		if i % SIZE == 0
		{
			if counts[0] < min
			{
				ans = counts[1] * counts[2];
				min = counts[0];
			}

			counts = [0, 0, 0];
		}
	}
	
	println!("{}", ans);
	
	Ok(())
}

