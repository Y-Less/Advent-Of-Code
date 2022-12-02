use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let input = "16,1,2,0,4,2,7,1,2,14";
    let numbers = input.split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

	// Least Squares (1d, y = 1).
	let mut sum = 0;
	for i in numbers
	{
		sum = sum + i;
	}

	let ave = sum as f64 / numbers.len() as f64;

	let mut numer: f64 = 0;
	let mut denom: f64 = 0;
	for i in numbers
	{
		let diff = (i as f64 - ave);
		numer = numer + diff;
		denom = denom + diff * diff;
	}

	
	
	
	

	let rt = (sum as f64).sqrt() / numbers.len;
	println!("{:?}", sum);
	println!("{:?}", rt as i64);
	
	Ok(())
}

