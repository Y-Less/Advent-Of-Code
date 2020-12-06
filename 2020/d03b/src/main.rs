use std::fs::File;
use std::io::prelude::*;

fn step(xs: usize, ys: usize, grid: &Vec<&[u8]>) -> i32
{
	let mut x = 0;
	let mut y = 0;
	let mut count = 0;
	let len = grid.len();
	loop
	{
		if y >= len
		{
			break;
		}
		let tree = grid[y][x % grid[y].len()];
		if tree == 35
		{
			count = count + 1;
		}
		y = y + ys;
		x = x + xs;
	}
	count
}

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d03.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let field = input.trim().split('\n')
		.map(|s| s.as_bytes())
		.collect::<Vec<&[u8]>>();

	//println!("{:?}", count);

	let a = step(1, 1, &field);
	let b = step(3, 1, &field);
	let c = step(5, 1, &field);
	let d = step(7, 1, &field);
	let e = step(1, 2, &field);

	
	println!("{:?}", a);
	println!("{:?}", b);
	println!("{:?}", c);
	println!("{:?}", d);
	println!("{:?}", e);
	println!("{:?}", a * b * c * d * e);
	
	Ok(())
}

