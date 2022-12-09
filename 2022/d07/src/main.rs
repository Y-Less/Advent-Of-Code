use std::fs::File;
use std::io::prelude::*;

trait Entry
{
	fn isDir(&self) -> bool;
	fn size(&self) -> usize;
	fn entries(&self) -> Vec<&dyn Entry>;
}

struct Directory
{
	files: Vec<dyn Entry>
}

struct Filename
{
	size: usize
}

impl Entry for Directory
{
	fn isDir(&self) -> bool
	{
		true
	}

	fn size(&self) -> usize
	{
		self.files.map(|&e| e.size()).sum()
	}

	fn entries(&self) -> Vec<&dyn Entry>
	{
		self.files
	}
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}

	let mut idx: usize;
	let mut idx = 0;
	let mut steps: [char; 14];
	steps = [ ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ' ];
	idx = 0;

	Ok(())
}

