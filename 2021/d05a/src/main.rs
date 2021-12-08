use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d05.txt")?;
		file.read_to_string(&mut input)?;
	}
	
	let sep = Regex::new(r"\s+->\s+|,").unwrap();
	let mut vents: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
	let mut lines = input.trim().split('\n');
	let mut dangerous = 0;
	for line in lines
	{
		let parts = sep.split(line).map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
		if (parts[0] != parts[2] && parts[1] != parts[3])
		{
			continue;
		}
		println!("{:?}", line);
		for i in std::cmp::min(parts[0], parts[2])..std::cmp::max(parts[0], parts[2]) + 1
		{
			for j in std::cmp::min(parts[1], parts[3])..std::cmp::max(parts[1], parts[3]) + 1
			{
				//println!("{:?} {:?}", i, j);
				if let Some(x) = vents.get_mut(&i)
				{
					if (x.contains_key(&j))
					{
						if (x[&j] == 1)
						{
							dangerous = dangerous + 1;
						}
						x.insert(j, x[&j] + 1);
					}
					else
					{
						x.insert(j, 1);
					}
				}
				else
				{
					let mut v = HashMap::new();
					v.insert(j, 1);
					vents.insert(i, v);
				}
				//if (!vents.contains_key(&i))
				//{
				//	vents[&i].insert(HashMap::new());
				//}
				//if (vents[&i].contains_key(&j))
				//{
				//	vents[&i][&j].insert(vents[&i][&j] + 1);
				//}
				//else
				//{
				//	vents[&i][&j] = 1;
				//}
			}
		}
	}
	//println!("{:?}", vents);

	//lines.next();
	//let mut board: [[i32; 5]; 5] = [[0; 5]; 5];
	//let mut i = 0;
	//loop
	//{
	//	match lines.next()
	//	{
	//		Some(line) =>
	//		{
	//			//println!("{:?}", line.split_whitespace().collect::<Vec<&str>>());
	//			
	//			let numbers: Vec<i32> = line.split_whitespace()
	//				.map(|x| x.parse().unwrap())
	//				.collect::<Vec<i32>>();
	//			if (numbers.len() == 0)
	//			{
	//				boards.push(board);
	//				board = [[0; 5]; 5];
	//				i = 0;
	//			}
	//			else
	//			{
	//				board[i][0] = numbers[0];
	//				board[i][1] = numbers[1];
	//				board[i][2] = numbers[2];
	//				board[i][3] = numbers[3];
	//				board[i][4] = numbers[4];
	//				i = i + 1;
	//			}
	//		}
	//		None =>
	//		{
	//			boards.push(board);
	//			break;
	//		}
	//	}
	//}
	//
	//let totalBoards = boards.len();
	//let mut remainingBoards = totalBoards;
	//let mut prev: Vec<i32> = Vec::new();
	//for i in 1..calls.len()
	//{
	//	let mut part: Vec<i32> = Vec::new();
	//	part.resize(i, 0);
	//	part.clone_from_slice(&calls[..i]);
	//	//println!("{:?}", part);
	//	let mut completeBoards = 0;
	//	for board in &boards
	//	{
	//		let sum = boardComplete(board, &part);
	//		if (sum != -1)
	//		{
	//			completeBoards = completeBoards + 1;
	//			if (remainingBoards == 1)
	//			{
	//				if (boardComplete(board, &prev) == -1)
	//				{
	//					let call = calls[i - 1];
	//					println!("{:?}", board);
	//					println!("{:?}", sum * call);
	//					return Ok(());
	//				}
	//			}
	//		}
	//	}
	//	prev = part;
	//	remainingBoards = totalBoards - completeBoards;
	//}
	
	Ok(())
}

