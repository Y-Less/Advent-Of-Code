use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d04.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	let mut total = 0;
	for line in lines
	{
		let pairs = line.trim().split(',').collect::<Vec<&str>>();
		let l = pairs[0].split('-')
			.map(|x| x.parse().expect("Not a number"))
			.collect::<Vec<i32>>();
		let r = pairs[1].split('-')
			.map(|x| x.parse().expect("Not a number"))
			.collect::<Vec<i32>>();
		if l[0] <= r[1] && l[1] >= r[0]
		{
			//println!("{:?} is fully contained in {:?}", r, l);
			total = total + 1;
		}
	}
	
	println!("{:?}", total);

	Ok(())
}

