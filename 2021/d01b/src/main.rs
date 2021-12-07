use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d01.txt")?;
		file.read_to_string(&mut input)?;
	}

	let numbers = input.trim().split('\n')
		.map(|x| x.parse().expect("Not a number"))
		.collect::<Vec<i32>>();

	let mut count = 0;
	
	let mut a = -1;
	let mut b = -1;
	let mut c = -1;
	let mut d = -1;

	for n in numbers
	{
		if (a == -1)
		{
			a = n;
		}
		else if (b == -1)
		{
			b = n;
		}
		else if (c == -1)
		{
			c = n;
		}
		else
		{
			// True sums.
			let prev = a + b + c;
			let cur = b + c + n;
			// Move the window.
			a = b;
			b = c;
			c = n;
			if (cur > prev)
			{
				count = count + 1;
			}
		}
	}

	println!("{:?}", count);

	Ok(())
}

