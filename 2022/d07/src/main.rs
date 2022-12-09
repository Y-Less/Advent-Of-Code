use std::fs::File;
use std::io::prelude::*;

struct Entry
{
	isDir: bool,
	size: usize,
	entries: Vec<Entry>,
}

impl Entry
{
	fn getSize(&self) -> usize
	{
		if self.isDir
		{
			self.entries.iter().map(|e| e.getSize()).sum()
		}
		else
		{
			self.size
		}
	}
}

fn listDir<'a>(name: &str, mut lines: impl std::iter::Iterator<Item = &'a str>) -> Entry
{
	//let x = lines.next();
	//println!("{:?}", x);
	Entry {
		isDir: false,
		size: 0,
		entries: vec!(),
	}
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let mut lines = input.trim().split('\n');
	lines.next();
	//let x = lines.next();
	//println!("{:?}", x);
	
	listDir("/", lines);
	
	//let mut stack

	Ok(())
}

