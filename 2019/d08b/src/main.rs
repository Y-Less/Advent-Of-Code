use std::fs::File;
use std::io::prelude::*;

const WIDTH: i32 = 25;
const SIZE: i32 = WIDTH * 6;

const C0: u8 = ' ' as u8;
const C1: u8 = '#' as u8;
const C2: u8 = '?' as u8;

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d08.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let layer: Vec<u8> = (0..WIDTH).map(|_x| C2).collect();
	let mut image: Vec<Vec<u8>> = [layer.clone(), layer.clone(), layer.clone(), layer.clone(), layer.clone(), layer.clone()].to_vec();

	for (i, p) in input.bytes().enumerate()
	{
		//let layer = i as i32 / SIZE;
		let pos = i as i32 % SIZE;
		let col = (pos % WIDTH) as usize;
		let row = (pos / WIDTH) as usize;

		if image[row][col] == C2 && p != 50
		{
			if p == 49
			{
				image[row][col] = C1;
			}
			else
			{
				image[row][col] = C0;
			}
		}
	}
	
	//println!("{:?}", image[0][6]);
	//println!("{:?}", image[1][6]);
	//println!("{:?}", image[2][6]);
	//println!("{:?}", image[3][6]);
	//println!("{:?}", image[4][6]);
	//println!("{:?}", image[5][6]);

	let row_0 = &image[0];
	let row_1 = &image[1];
	let row_2 = &image[2];
	let row_3 = &image[3];
	let row_4 = &image[4];
	let row_5 = &image[5];

	let row_0 = String::from_utf8(row_0.to_vec()).expect("");
	let row_1 = String::from_utf8(row_1.to_vec()).expect("");
	let row_2 = String::from_utf8(row_2.to_vec()).expect("");
	let row_3 = String::from_utf8(row_3.to_vec()).expect("");
	let row_4 = String::from_utf8(row_4.to_vec()).expect("");
	let row_5 = String::from_utf8(row_5.to_vec()).expect("");
	
	println!("{}", row_0);
	println!("{}", row_1);
	println!("{}", row_2);
	println!("{}", row_3);
	println!("{}", row_4);
	println!("{}", row_5);
	
	Ok(())
}

