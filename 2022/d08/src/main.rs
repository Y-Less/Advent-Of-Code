use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let input = vec!(
		vec!(3, 0, 3, 7, 3),
		vec!(2, 5, 5, 1, 2),
		vec!(6, 5, 3, 3, 2),
		vec!(3, 3, 5, 4, 9),
		vec!(3, 5, 3, 9, 0)
	);

	let w = input.len();
	let h = input[0].len();
	let mut visibility = vec![vec![0; h]; w];
    
	println!("Debug A: {:?}", input);
	
	for i in 0..w
	{
		let mut highest = -1;
		for j in 0..h
		{
			let cur = input[i][j];
			if cur > highest
			{
				highest = cur;
				visibility[i][j] = visibility[i][j] | 1;
			}
		}
	}
	
	println!("Debug A: {:?}", visibility);

	Ok(())
}

