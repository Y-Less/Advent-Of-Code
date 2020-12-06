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

	let prog = input.trim().split('\n')
		.map(|x|
		{
			let (l, u, c, s) = scan_fmt!(x, "{d}-{d} {[a-z]}: {s}\n", i32, i32, String, String).unwrap();
			println!("{} {} {} {}", l, u, c, s);
			(l, u, c, s)
		})
		.collect::<Vec<(i32, i32, String, String)>>();
	
	let mut valid = 0;
	for p in prog
	{
		let cmp = p.2.as_bytes()[0];
		let str = p.3.as_bytes();
		let chk = (str[(p.0 - 1) as usize] == cmp) != (str[(p.1 - 1) as usize] == cmp);
		
		if chk
		{
			valid = valid + 1
		}
		println!("chk = {}", chk);
	}
	println!("valid = {}", valid);

	Ok(())
}

