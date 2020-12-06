use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

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
		if row == "FFFFFFB" { rr = 1  ; }
		if row == "FFFFFBF" { rr = 2  ; }
		if row == "FFFFFBB" { rr = 3  ; }
		if row == "FFFFBFF" { rr = 4  ; }
		if row == "FFFFBFB" { rr = 5  ; }
		if row == "FFFFBBF" { rr = 6  ; }
		if row == "FFFFBBB" { rr = 7  ; }
		if row == "FFFBFFF" { rr = 8  ; }
		if row == "FFFBFFB" { rr = 9  ; }
		if row == "FFFBFBF" { rr = 10 ; }
		if row == "FFFBFBB" { rr = 11 ; }
		if row == "FFFBBFF" { rr = 12 ; }
		if row == "FFFBBFB" { rr = 13 ; }
		if row == "FFFBBBF" { rr = 14 ; }
		if row == "FFFBBBB" { rr = 15 ; }
		if row == "FFBFFFF" { rr = 16 ; }
		if row == "FFBFFFB" { rr = 17 ; }
		if row == "FFBFFBF" { rr = 18 ; }
		if row == "FFBFFBB" { rr = 19 ; }
		if row == "FFBFBFF" { rr = 20 ; }
		if row == "FFBFBFB" { rr = 21 ; }
		if row == "FFBFBBF" { rr = 22 ; }
		if row == "FFBFBBB" { rr = 23 ; }
		if row == "FFBBFFF" { rr = 24 ; }
		if row == "FFBBFFB" { rr = 25 ; }
		if row == "FFBBFBF" { rr = 26 ; }
		if row == "FFBBFBB" { rr = 27 ; }
		if row == "FFBBBFF" { rr = 28 ; }
		if row == "FFBBBFB" { rr = 29 ; }
		if row == "FFBBBBF" { rr = 30 ; }
		if row == "FFBBBBB" { rr = 31 ; }
		if row == "FBFFFFF" { rr = 32 ; }
		if row == "FBFFFFB" { rr = 33 ; }
		if row == "FBFFFBF" { rr = 34 ; }
		if row == "FBFFFBB" { rr = 35 ; }
		if row == "FBFFBFF" { rr = 36 ; }
		if row == "FBFFBFB" { rr = 37 ; }
		if row == "FBFFBBF" { rr = 38 ; }
		if row == "FBFFBBB" { rr = 39 ; }
		if row == "FBFBFFF" { rr = 40 ; }
		if row == "FBFBFFB" { rr = 41 ; }
		if row == "FBFBFBF" { rr = 42 ; }
		if row == "FBFBFBB" { rr = 43 ; }
		if row == "FBFBBFF" { rr = 44 ; }
		if row == "FBFBBFB" { rr = 45 ; }
		if row == "FBFBBBF" { rr = 46 ; }
		if row == "FBFBBBB" { rr = 47 ; }
		if row == "FBBFFFF" { rr = 48 ; }
		if row == "FBBFFFB" { rr = 49 ; }
		if row == "FBBFFBF" { rr = 50 ; }
		if row == "FBBFFBB" { rr = 51 ; }
		if row == "FBBFBFF" { rr = 52 ; }
		if row == "FBBFBFB" { rr = 53 ; }
		if row == "FBBFBBF" { rr = 54 ; }
		if row == "FBBFBBB" { rr = 55 ; }
		if row == "FBBBFFF" { rr = 56 ; }
		if row == "FBBBFFB" { rr = 57 ; }
		if row == "FBBBFBF" { rr = 58 ; }
		if row == "FBBBFBB" { rr = 59 ; }
		if row == "FBBBBFF" { rr = 60 ; }
		if row == "FBBBBFB" { rr = 61 ; }
		if row == "FBBBBBF" { rr = 62 ; }
		if row == "FBBBBBB" { rr = 63 ; }
		if row == "BFFFFFF" { rr = 64 ; }
		if row == "BFFFFFB" { rr = 65 ; }
		if row == "BFFFFBF" { rr = 66 ; }
		if row == "BFFFFBB" { rr = 67 ; }
		if row == "BFFFBFF" { rr = 68 ; }
		if row == "BFFFBFB" { rr = 69 ; }
		if row == "BFFFBBF" { rr = 70 ; }
		if row == "BFFFBBB" { rr = 71 ; }
		if row == "BFFBFFF" { rr = 72 ; }
		if row == "BFFBFFB" { rr = 73 ; }
		if row == "BFFBFBF" { rr = 74 ; }
		if row == "BFFBFBB" { rr = 75 ; }
		if row == "BFFBBFF" { rr = 76 ; }
		if row == "BFFBBFB" { rr = 77 ; }
		if row == "BFFBBBF" { rr = 78 ; }
		if row == "BFFBBBB" { rr = 79 ; }
		if row == "BFBFFFF" { rr = 80 ; }
		if row == "BFBFFFB" { rr = 81 ; }
		if row == "BFBFFBF" { rr = 82 ; }
		if row == "BFBFFBB" { rr = 83 ; }
		if row == "BFBFBFF" { rr = 84 ; }
		if row == "BFBFBFB" { rr = 85 ; }
		if row == "BFBFBBF" { rr = 86 ; }
		if row == "BFBFBBB" { rr = 87 ; }
		if row == "BFBBFFF" { rr = 88 ; }
		if row == "BFBBFFB" { rr = 89 ; }
		if row == "BFBBFBF" { rr = 90 ; }
		if row == "BFBBFBB" { rr = 91 ; }
		if row == "BFBBBFF" { rr = 92 ; }
		if row == "BFBBBFB" { rr = 93 ; }
		if row == "BFBBBBF" { rr = 94 ; }
		if row == "BFBBBBB" { rr = 95 ; }
		if row == "BBFFFFF" { rr = 96 ; }
		if row == "BBFFFFB" { rr = 97 ; }
		if row == "BBFFFBF" { rr = 98 ; }
		if row == "BBFFFBB" { rr = 99 ; }
		if row == "BBFFBFF" { rr = 100; }
		if row == "BBFFBFB" { rr = 101; }
		if row == "BBFFBBF" { rr = 102; }
		if row == "BBFFBBB" { rr = 103; }
		if row == "BBFBFFF" { rr = 104; }
		if row == "BBFBFFB" { rr = 105; }
		if row == "BBFBFBF" { rr = 106; }
		if row == "BBFBFBB" { rr = 107; }
		if row == "BBFBBFF" { rr = 108; }
		if row == "BBFBBFB" { rr = 109; }
		if row == "BBFBBBF" { rr = 110; }
		if row == "BBFBBBB" { rr = 111; }
		if row == "BBBFFFF" { rr = 112; }
		if row == "BBBFFFB" { rr = 113; }
		if row == "BBBFFBF" { rr = 114; }
		if row == "BBBFFBB" { rr = 115; }
		if row == "BBBFBFF" { rr = 116; }
		if row == "BBBFBFB" { rr = 117; }
		if row == "BBBFBBF" { rr = 118; }
		if row == "BBBFBBB" { rr = 119; }
		if row == "BBBBFFF" { rr = 120; }
		if row == "BBBBFFB" { rr = 121; }
		if row == "BBBBFBF" { rr = 122; }
		if row == "BBBBFBB" { rr = 123; }
		if row == "BBBBBFF" { rr = 124; }
		if row == "BBBBBFB" { rr = 125; }
		if row == "BBBBBBF" { rr = 126; }
		if row == "BBBBBBB" { rr = 127; }

		if col == "LLL" { cc = 0  ; }
		if col == "LLR" { cc = 1  ; }
		if col == "LRL" { cc = 2  ; }
		if col == "LRR" { cc = 3  ; }
		if col == "RLL" { cc = 4  ; }
		if col == "RLR" { cc = 5  ; }
		if col == "RRL" { cc = 6  ; }
		if col == "RRR" { cc = 7  ; }

		let id = rr * 8 + cc;
		if id > max
		{
			max = id;
		}
		println!("{:?}", rr * 8 + cc);
	}
	println!("{:?}", max);

	Ok(())
}

