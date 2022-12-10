use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>
{
	let input = vec!(
		vec!(3, 0, 3, 7, 3),
		vec!(2, 5, 5, 1, 2),
		vec!(6, 5, 3, 3, 2),
		vec!(3, 3, 5, 4, 9),
		vec!(3, 5, 3, 9, 0)
	);

	let visibility = vec![vec![0; input[0].len()]; input.len()];
    
	println!("Debug A: {:?}", input);
	println!("Debug A: {:?}", visibility);

	Ok(())
}

