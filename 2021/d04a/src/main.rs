use std::fs::File;
use std::io::prelude::*;

fn readNumbers<'a, P>(mut lines: std::str::Split<'a, P>)
{
	let line = lines.next();
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d04.txt")?;
		file.read_to_string(&mut input)?;
	}

	Ok(())
}

