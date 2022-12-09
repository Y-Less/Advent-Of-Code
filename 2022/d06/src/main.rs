use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}

	let mut idx = 0;
	let mut a = ' ';
	let mut b = ' ';
	let mut c = ' ';
	let mut d = ' ';
	for ch in input.chars()
	{
		idx = idx + 1;
		a = b;
		b = c;
		c = d;
		d = ch;
		if (idx >= 4)
		{
			if (a != b && a != c && a != d && b != c && b != d && c != d)
			{
				println!("Part A: {:?} {:?} {:?} {:?} {:?}", idx, a, b, c, d);
				break;
			}
		}
	}

	Ok(())
}

