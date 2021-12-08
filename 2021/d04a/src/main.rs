use std::fs::File;
use std::io::prelude::*;

fn readNumbers(line: &str) -> Vec<i32>
{
	let numbers: Vec<i32> = line.trim().split(',')
		.map(|x| x.parse().unwrap())
		.collect::<Vec<i32>>();

	numbers
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d04.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let mut boards: Vec<[[i32; 5]; 5]> = vec!();

	let mut lines = input.trim().split('\n');
	
	let calls = readNumbers(lines.next().unwrap());
	println!("{:?}", calls);
	
	lines.next();
	let mut board: [[i32; 5]; 5] = [[0; 5]; 5];
	let mut i = 0;
	loop
	{
		match lines.next()
		{
			Some(line) =>
			{
				//println!("{:?}", line.split_whitespace().collect::<Vec<&str>>());
				
				let numbers: Vec<i32> = line.split_whitespace()
					.map(|x| x.parse().unwrap())
					.collect::<Vec<i32>>();
				if (numbers.len() == 0)
				{
					boards.push(board);
					board = [[0; 5]; 5];
					i = 0;
				}
				else
				{
					board[i][0] = numbers[0];
					board[i][1] = numbers[1];
					board[i][2] = numbers[2];
					board[i][3] = numbers[3];
					board[i][4] = numbers[4];
					i = i + 1;
				}
			}
			None =>
			{
				break;
			}
		}
	}
	println!("{:?}", boards);
	
	Ok(())
}

