use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	// Yeah, I can't be bothered to write a parser for this lot!
	//
	//           [H]     [W] [B]            
	//       [D] [B]     [L] [G] [N]        
	//   [P] [J] [T]     [M] [R] [D]        
	//   [V] [F] [V]     [F] [Z] [B]     [C]
	//   [Z] [V] [S]     [G] [H] [C] [Q] [R]
	//   [W] [W] [L] [J] [B] [V] [P] [B] [Z]
	//   [D] [S] [M] [S] [Z] [W] [J] [T] [G]
	//   [T] [L] [Z] [R] [C] [Q] [V] [P] [H]
	//    1   2   3   4   5   6   7   8   9 
	//
	let mut stacks: Vec<Vec<char>> = vec!(
		vec!('T', 'D', 'W', 'Z', 'V', 'P'),
		vec!('L', 'S', 'W', 'V', 'F', 'J', 'D'),
		vec!('Z', 'M', 'L', 'S', 'V', 'T', 'B', 'H'),
		vec!('R', 'S', 'J'),
		vec!('C', 'Z', 'B', 'G', 'F', 'M', 'L', 'W'),
		vec!('Q', 'W', 'V', 'H', 'Z', 'R', 'G', 'B'),
		vec!('V', 'J', 'P', 'C', 'B', 'D', 'N'),
		vec!('P', 'T', 'B', 'Q'),
		vec!('H', 'G', 'Z', 'R', 'C'),
	);

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d05.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	for line in lines
	{
		let m = line.trim().split(' ').collect::<Vec<&str>>();
		let n: i32 = m[1].parse::<i32>().expect("Not a number");
		let f: usize = m[3].parse::<usize>().expect("Not a number") - 1;
		let t: usize = m[5].parse::<usize>().expect("Not a number") - 1;
		let mut tmp: Vec<char> = vec!();
		for _i in 0..n
		{
			let c = stacks[f].pop();
			tmp.push(c.expect("No crate"));
		}
		for _i in 0..n
		{
			let c = tmp.pop();
			stacks[t].push(c.expect("No crate"));
		}
	}
	for _i in 0..8+1
	{
		let c = stacks[_i].pop();
		println!("{:?}", c);
	}

	Ok(())
}

