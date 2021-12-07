use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d03.txt")?;
		file.read_to_string(&mut input)?;
	}
	let numbers = input.trim().split('\n')
		.map(|x| i32::from_str_radix(x, 2).unwrap())
		.collect::<Vec<i32>>();

	println!("{:?}", numbers);

	// Oxygen.
	let mut oxygen = 0;
	let mut nu = numbers.to_vec();
	for i in 0..5
	{
		let mut naughts = 0;
		let mut ones = 0;

		let b = 1 << (4 - i);
		for n in nu.iter()
		{
			if ((n & b) == 0)
			{
				naughts = naughts + 1;
			}
			else
			{
				ones = ones + 1;
			}
		}

		if (ones >= naughts)
		{
			nu.retain(|&x| x & b != 0)
		}
		else
		{
			nu.retain(|&x| x & b == 0)
		}
	}
	println!("{:?}", nu);

	//let mut gamma = 0;
	//let mut epsilon = 0;
	//for i in 0..5
	//{
	//	let b = 1 << i;
	//	if (ones[i] >= naughts[i])
	//	{
	//		gamma = gamma | b;
	//	}
	//	else
	//	{
	//		epsilon = epsilon | b;
	//	}
	//}
    //
	//println!("{:?}", epsilon);
	//println!("{:?}", gamma * epsilon);

	Ok(())
}

