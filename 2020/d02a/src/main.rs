use std::fs::File;
use std::io::prelude::*;
#[macro_use] extern crate scan_fmt;

fn main() -> std::io::Result<()>
{
    println!("Enter the program.");
	
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d02.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

//	let mut prog = String::new();
//	
//	stdin.read_line(&mut prog)
//		.expect("Please enter something");
//	
	let prog = input.trim().split('\n')
		.map(|x|
		{
			let (l, u, c, s) = scan_fmt!(x, "{d}-{d} {[a-z]}: {s}\n", i32, i32, String, String).unwrap();
			println!("{} {} {} {}", l, u, c, s);
			(l, u, c, s)
		})
		.collect::<Vec<(i32, i32, String, String)>>();
	//let prog = prog.split(' ');//.collect::<Vec<i32>>();
	
	for p in prog
	{
		println!("{:?}", p);
	}
	
//
//
//	//loop
//	//{
//	//	let num: i32 = match num.trim().parse()
//	//	{
//	//		Ok(n) => n,
//	//		Err(_) => break,
//	//	};
//	//	
//	//		//.expect("Bye");
//	//	
//	//	vec.push(num);
//	//	
//	//	//let calc: i32 = num / 3 - 2;
//	//	//println!("Number: {}, {}", num, calc);
//	//}
//	//
//	//let vec = vec.iter().map(fuel);
//
////	prog.iter().for_each(|&x| { println!("{}", &x); x = 7; });
////	prog.iter().map(|&x| 7).for_each(|x| println!("{}", x));
//	//prog.for_each(|x| println!("{}", x));
//	//let total: i32 = vec.fold(0, |cur, x| cur + x);
//	//
//	//
//	//println!("{}", total);
//	
//	
//	let mut i = 0;
//	let len = prog.len();
//	
//	loop
//	{
//		if i >= len
//		{
//			return;
//		}
//		
//		match prog[i]
//		{
//			1 =>
//			{
//				let i0 = prog[i + 3] as usize;
//				let i1 = prog[i + 1] as usize;
//				let i2 = prog[i + 2] as usize;
//				let total = prog[i1] + prog[i2];
//				prog[i0] = total;
//				i += 4;
//			}
//			2 =>
//			{
//				let i0 = prog[i + 3] as usize;
//				let i1 = prog[i + 1] as usize;
//				let i2 = prog[i + 2] as usize;
//				let total = prog[i1] * prog[i2];
//				prog[i0] = total;
//				i += 4;
//			}
//			99 => break,
//			_ =>
//			{
//				i += 1;
//			}
//		}
//	}
//	
//	prog.iter().for_each(|x| println!("{}", &x) );

	Ok(())
}

