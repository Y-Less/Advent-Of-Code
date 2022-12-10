use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Entry
{
	name: String,
	isDir: bool,
	size: usize,
	entries: Vec<Entry>,
}

impl Entry
{
	fn getSmallest(&self, mut smallest: usize, required: usize) -> usize
	{
		if self.isDir
		{
			for e in self.entries.iter()
			{
				smallest = e.getSmallest(smallest, required);
			}
			let s2 = self.getSize();
			if s2 >= required && s2 < smallest
			{
				s2
			}
			else
			{
				smallest
			}
		}
		else
		{
			smallest
		}
	}

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

	fn getSmallDirs(&self) -> usize
	{
		if self.isDir
		{
			let sub: usize = self.entries.iter().map(|e| e.getSmallDirs()).sum();
			let cur: usize = self.entries.iter().map(|e| e.getSize()).sum();
			if cur <= 100000
			{
				cur + sub
			}
			else
			{
				sub
			}
		}
		else
		{
			0
		}
	}
}

fn listDir<'a>(name: &str, lines: &Vec<&str>, idx: &mut usize) -> Entry
{
	//let x = lines.next();
	//println!("name: {:?}, idx: {:?}", name, idx);
	let mut entries: Vec<Entry> = vec!();
	
	//for line in lines
	loop
	{
		let moo = lines.get(*idx);
		if moo == None
		{
			break;
		}
		let line = *moo.unwrap();
		*idx = *idx + 1;
		if line == "$ ls"
		{
			// Ignore this.
			//println!("ls");
		}
		else if line == "$ cd .."
		{
			//println!("back from {:?}", name);
			break;
		}
		else if line.chars().nth(0) == Some('$')
		{
			//println!("{:?}", line.get(5..));
			entries.push(listDir(line.get(5..).unwrap(), lines, idx));
		}
		else if line.get(0..4) == Some("dir ")
		{
			// Add an entry.
			//println!("dir: {:?}", line.get(4..));
		}
		else
		{
			// Add an entry.
			let mut bits = line.split(' ');
			let a = bits.nth(0).unwrap();
			let b = bits.nth(0).unwrap();
			//println!("file: {:?}, {:?}", b, a);
			entries.push(Entry {
				name: b.to_string(),
				size: a.parse::<usize>().expect("Not a number"),
				isDir: false,
				entries: vec!(),
			});
		}
	}
	
	Entry {
		name: name.to_string(),
		isDir: true,
		size: 0,
		entries,
	}
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("input.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let mut lines = input.trim().split('\n').collect::<Vec<&str>>();
	let mut idx: usize = 1;
	
	let dirs = listDir("/", &lines, &mut idx);
	println!("dirs: {:?}", dirs);
	
	println!("Part A: {:?}", dirs.getSmallDirs());
	let available = 70000000 - dirs.getSize();
	let required = 30000000 - available;
	println!("Size B: {:?} {:?}", available, required);
	let smallest = dirs.getSmallest(70000000, required);
	println!("Part B: {:?}", smallest);

	Ok(())
}

