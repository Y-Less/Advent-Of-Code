use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	println!("Enter the program.");

	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d05.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let list = input.split("\n")
		.collect::<Vec<&str>>();
	let mut max = 0;
	let mut seats: [bool; 1024] = [false; 1024];
	for p in list
	{
		//println!("{}", p);
		let row = &p[..7];
		let col = &p[7..];
		let mut rr = 0;
		let mut cc = 0;
		//println!("{:?}", row);
		//println!("{:?}", col);
	
		if row == "FFFFFFF" { rr = 0  ; }
		else if row == "FFFFFFB" { rr = 1  ; }
		else if row == "FFFFFBF" { rr = 2  ; }
		else if row == "FFFFFBB" { rr = 3  ; }
		else if row == "FFFFBFF" { rr = 4  ; }
		else if row == "FFFFBFB" { rr = 5  ; }
		else if row == "FFFFBBF" { rr = 6  ; }
		else if row == "FFFFBBB" { rr = 7  ; }
		else if row == "FFFBFFF" { rr = 8  ; }
		else if row == "FFFBFFB" { rr = 9  ; }
		else if row == "FFFBFBF" { rr = 10 ; }
		else if row == "FFFBFBB" { rr = 11 ; }
		else if row == "FFFBBFF" { rr = 12 ; }
		else if row == "FFFBBFB" { rr = 13 ; }
		else if row == "FFFBBBF" { rr = 14 ; }
		else if row == "FFFBBBB" { rr = 15 ; }
		else if row == "FFBFFFF" { rr = 16 ; }
		else if row == "FFBFFFB" { rr = 17 ; }
		else if row == "FFBFFBF" { rr = 18 ; }
		else if row == "FFBFFBB" { rr = 19 ; }
		else if row == "FFBFBFF" { rr = 20 ; }
		else if row == "FFBFBFB" { rr = 21 ; }
		else if row == "FFBFBBF" { rr = 22 ; }
		else if row == "FFBFBBB" { rr = 23 ; }
		else if row == "FFBBFFF" { rr = 24 ; }
		else if row == "FFBBFFB" { rr = 25 ; }
		else if row == "FFBBFBF" { rr = 26 ; }
		else if row == "FFBBFBB" { rr = 27 ; }
		else if row == "FFBBBFF" { rr = 28 ; }
		else if row == "FFBBBFB" { rr = 29 ; }
		else if row == "FFBBBBF" { rr = 30 ; }
		else if row == "FFBBBBB" { rr = 31 ; }
		else if row == "FBFFFFF" { rr = 32 ; }
		else if row == "FBFFFFB" { rr = 33 ; }
		else if row == "FBFFFBF" { rr = 34 ; }
		else if row == "FBFFFBB" { rr = 35 ; }
		else if row == "FBFFBFF" { rr = 36 ; }
		else if row == "FBFFBFB" { rr = 37 ; }
		else if row == "FBFFBBF" { rr = 38 ; }
		else if row == "FBFFBBB" { rr = 39 ; }
		else if row == "FBFBFFF" { rr = 40 ; }
		else if row == "FBFBFFB" { rr = 41 ; }
		else if row == "FBFBFBF" { rr = 42 ; }
		else if row == "FBFBFBB" { rr = 43 ; }
		else if row == "FBFBBFF" { rr = 44 ; }
		else if row == "FBFBBFB" { rr = 45 ; }
		else if row == "FBFBBBF" { rr = 46 ; }
		else if row == "FBFBBBB" { rr = 47 ; }
		else if row == "FBBFFFF" { rr = 48 ; }
		else if row == "FBBFFFB" { rr = 49 ; }
		else if row == "FBBFFBF" { rr = 50 ; }
		else if row == "FBBFFBB" { rr = 51 ; }
		else if row == "FBBFBFF" { rr = 52 ; }
		else if row == "FBBFBFB" { rr = 53 ; }
		else if row == "FBBFBBF" { rr = 54 ; }
		else if row == "FBBFBBB" { rr = 55 ; }
		else if row == "FBBBFFF" { rr = 56 ; }
		else if row == "FBBBFFB" { rr = 57 ; }
		else if row == "FBBBFBF" { rr = 58 ; }
		else if row == "FBBBFBB" { rr = 59 ; }
		else if row == "FBBBBFF" { rr = 60 ; }
		else if row == "FBBBBFB" { rr = 61 ; }
		else if row == "FBBBBBF" { rr = 62 ; }
		else if row == "FBBBBBB" { rr = 63 ; }
		else if row == "BFFFFFF" { rr = 64 ; }
		else if row == "BFFFFFB" { rr = 65 ; }
		else if row == "BFFFFBF" { rr = 66 ; }
		else if row == "BFFFFBB" { rr = 67 ; }
		else if row == "BFFFBFF" { rr = 68 ; }
		else if row == "BFFFBFB" { rr = 69 ; }
		else if row == "BFFFBBF" { rr = 70 ; }
		else if row == "BFFFBBB" { rr = 71 ; }
		else if row == "BFFBFFF" { rr = 72 ; }
		else if row == "BFFBFFB" { rr = 73 ; }
		else if row == "BFFBFBF" { rr = 74 ; }
		else if row == "BFFBFBB" { rr = 75 ; }
		else if row == "BFFBBFF" { rr = 76 ; }
		else if row == "BFFBBFB" { rr = 77 ; }
		else if row == "BFFBBBF" { rr = 78 ; }
		else if row == "BFFBBBB" { rr = 79 ; }
		else if row == "BFBFFFF" { rr = 80 ; }
		else if row == "BFBFFFB" { rr = 81 ; }
		else if row == "BFBFFBF" { rr = 82 ; }
		else if row == "BFBFFBB" { rr = 83 ; }
		else if row == "BFBFBFF" { rr = 84 ; }
		else if row == "BFBFBFB" { rr = 85 ; }
		else if row == "BFBFBBF" { rr = 86 ; }
		else if row == "BFBFBBB" { rr = 87 ; }
		else if row == "BFBBFFF" { rr = 88 ; }
		else if row == "BFBBFFB" { rr = 89 ; }
		else if row == "BFBBFBF" { rr = 90 ; }
		else if row == "BFBBFBB" { rr = 91 ; }
		else if row == "BFBBBFF" { rr = 92 ; }
		else if row == "BFBBBFB" { rr = 93 ; }
		else if row == "BFBBBBF" { rr = 94 ; }
		else if row == "BFBBBBB" { rr = 95 ; }
		else if row == "BBFFFFF" { rr = 96 ; }
		else if row == "BBFFFFB" { rr = 97 ; }
		else if row == "BBFFFBF" { rr = 98 ; }
		else if row == "BBFFFBB" { rr = 99 ; }
		else if row == "BBFFBFF" { rr = 100; }
		else if row == "BBFFBFB" { rr = 101; }
		else if row == "BBFFBBF" { rr = 102; }
		else if row == "BBFFBBB" { rr = 103; }
		else if row == "BBFBFFF" { rr = 104; }
		else if row == "BBFBFFB" { rr = 105; }
		else if row == "BBFBFBF" { rr = 106; }
		else if row == "BBFBFBB" { rr = 107; }
		else if row == "BBFBBFF" { rr = 108; }
		else if row == "BBFBBFB" { rr = 109; }
		else if row == "BBFBBBF" { rr = 110; }
		else if row == "BBFBBBB" { rr = 111; }
		else if row == "BBBFFFF" { rr = 112; }
		else if row == "BBBFFFB" { rr = 113; }
		else if row == "BBBFFBF" { rr = 114; }
		else if row == "BBBFFBB" { rr = 115; }
		else if row == "BBBFBFF" { rr = 116; }
		else if row == "BBBFBFB" { rr = 117; }
		else if row == "BBBFBBF" { rr = 118; }
		else if row == "BBBFBBB" { rr = 119; }
		else if row == "BBBBFFF" { rr = 120; }
		else if row == "BBBBFFB" { rr = 121; }
		else if row == "BBBBFBF" { rr = 122; }
		else if row == "BBBBFBB" { rr = 123; }
		else if row == "BBBBBFF" { rr = 124; }
		else if row == "BBBBBFB" { rr = 125; }
		else if row == "BBBBBBF" { rr = 126; }
		else if row == "BBBBBBB" { rr = 127; }

		if col == "LLL" { cc = 0  ; }
		else if col == "LLR" { cc = 1  ; }
		else if col == "LRL" { cc = 2  ; }
		else if col == "LRR" { cc = 3  ; }
		else if col == "RLL" { cc = 4  ; }
		else if col == "RLR" { cc = 5  ; }
		else if col == "RRL" { cc = 6  ; }
		else if col == "RRR" { cc = 7  ; }

		let id = rr * 8 + cc;
		if id > max
		{
			max = id;
		}
		seats[id] = true;
		println!("{:?}", rr * 8 + cc);
	}
	println!("{:?}", max);
	
	let mut last = false;
	for i in 0..1024
	{
		if seats[i]
		{
			last = true;
		}
		else if last
		{
			println!("{:?}", i);
		}
	}
	

	Ok(())
}

