use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let input = "3,4,3,1,2";

	// Initialise the array.
	let mut cycles: [i32; 9] = [0; 9];
	input.split(',').for_each(|x| 
	{
		let n: usize = x.parse().unwrap();
		cycles[n] = cycles[n] + 1;
	});

	println!("{:?}", cycles);

	for i in 0..80
	{
		// Step
		let nu = cycles[0];
		cycles[0] = cycles[1];
		cycles[1] = cycles[2];
		cycles[2] = cycles[3];
		cycles[3] = cycles[4];
		cycles[4] = cycles[5];
		cycles[5] = cycles[6];
		cycles[6] = cycles[7] + nu;
		cycles[7] = cycles[8];
		cycles[8] = nu;
	}

	let mut sum = 0;
	for i in 0..9
	{
		// Step
		let nu = cycles[0];
		sum = sum + cycles[i];
	}

	println!("{:?}", sum);
	
	Ok(())
}

