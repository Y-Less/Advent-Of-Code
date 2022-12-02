use std::fs::File;
use std::io::prelude::*;

fn score(them: &str, you: &str) -> i32
{
	match you
	{
		"X" => {
			// Rock = 1
			match them
			{
				"A" => { 4 } // Draw = 3
				"B" => { 1 } // Loss = 0
				"C" => { 7 } // Win = 6
				_   => { 0 }
			}
		}
		"Y" => {
			// Paper = 2
			match them
			{
				"A" => { 8 }
				"B" => { 5 }
				"C" => { 2 }
				_   => { 0 }
			}
		}
		"Z" => {
			// Scissors = 3
			match them
			{
				"A" => { 3 }
				"B" => { 9 }
				"C" => { 6 }
				_   => { 0 }
			}
		}
		_   => { 0 }
	}
}

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d02.txt")?;
		file.read_to_string(&mut input)?;
	}

	let lines = input.trim().split('\n');
	let mut total = 0;
	for line in lines
	{
		let original = line.trim().split(' ').collect::<Vec<&str>>();
		//println!("{:?}", original[0]);
		total = total + score(original[0], original[1]);
		println!("{:?}", score(original[0], original[1]));
	}
	println!("{:?}", total);

	Ok(())
}

