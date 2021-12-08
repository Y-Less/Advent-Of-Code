use std::fs::File;
use std::io::prelude::*;

fn readNumbers(line: &str) -> Vec<i32>
{
	let numbers: Vec<i32> = line.trim().split(',')
		.map(|x| x.parse().unwrap())
		.collect::<Vec<i32>>();

	numbers
}

fn boardComplete(board: [[i32; 5]; 5], numbers: Vec<i32>) -> i32
{
	if
		(numbers.contains(&board[0][0]) && numbers.contains(&board[0][1]) && numbers.contains(&board[0][2]) && numbers.contains(&board[0][3]) && numbers.contains(&board[0][4])) ||
		(numbers.contains(&board[1][0]) && numbers.contains(&board[1][1]) && numbers.contains(&board[1][2]) && numbers.contains(&board[1][3]) && numbers.contains(&board[1][4])) ||
		(numbers.contains(&board[2][0]) && numbers.contains(&board[2][1]) && numbers.contains(&board[2][2]) && numbers.contains(&board[2][3]) && numbers.contains(&board[2][4])) ||
		(numbers.contains(&board[3][0]) && numbers.contains(&board[3][1]) && numbers.contains(&board[3][2]) && numbers.contains(&board[3][3]) && numbers.contains(&board[3][4])) ||
		(numbers.contains(&board[4][0]) && numbers.contains(&board[4][1]) && numbers.contains(&board[4][2]) && numbers.contains(&board[4][3]) && numbers.contains(&board[4][4]))
	{
		// Horizontals.
	}
	else if
		(numbers.contains(&board[0][0]) && numbers.contains(&board[1][0]) && numbers.contains(&board[2][0]) && numbers.contains(&board[3][0]) && numbers.contains(&board[4][0])) ||
		(numbers.contains(&board[0][1]) && numbers.contains(&board[1][1]) && numbers.contains(&board[2][1]) && numbers.contains(&board[3][1]) && numbers.contains(&board[4][1])) ||
		(numbers.contains(&board[0][2]) && numbers.contains(&board[1][2]) && numbers.contains(&board[2][2]) && numbers.contains(&board[3][2]) && numbers.contains(&board[4][2])) ||
		(numbers.contains(&board[0][3]) && numbers.contains(&board[1][3]) && numbers.contains(&board[2][3]) && numbers.contains(&board[3][3]) && numbers.contains(&board[4][3])) ||
		(numbers.contains(&board[0][4]) && numbers.contains(&board[1][4]) && numbers.contains(&board[2][4]) && numbers.contains(&board[3][4]) && numbers.contains(&board[4][4]))
	{
		// Verticals.
	}
	else
	{
		return -1;
	}
	
	// Sum the board.
	let mut sum = 0;
	for i in 0..5
	{
		for j in 0..5
		{
			if (!numbers.contains(&board[i][j]))
			{
				sum = sum + board[i][j];
			}
		}
	}
	
	sum
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
				boards.push(board);
				break;
			}
		}
	}
	println!("{:?}", boards);
	
	Ok(())
}

