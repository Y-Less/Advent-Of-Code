//#![feature(generators, generator_trait)]
//
//use std::ops::{Generator, GeneratorState};
//use std::pin::Pin;
use std::vec::Vec;

fn get_pattern(repeats: usize, mem: &mut Vec<i32>) -> std::iter::Cycle<std::slice::Iter<i32>> //-> &'a dyn Iterator<Item = &i32>
{
	mem.clear();
	for _ in 0 .. repeats
	{
		mem.push(0);
	}
	for _ in 0 .. repeats
	{
		mem.push(1);
	}
	for _ in 0 .. repeats
	{
		mem.push(0);
	}
	for _ in 0 .. repeats
	{
		mem.push(-1);
	}
	//println!("{:?}", mem);
	let mut ret = mem.iter().cycle();
	ret.next();
	return ret;
}

const PHASES: i32 = 100;
const LEN: usize = 32;

fn main()
{

	let input = "69317163492948606335995924319873";
	let mut output: [i32; LEN] = [0; LEN];
	for (i, ch) in input.bytes().enumerate()
	{
		output[i] = ch as i32 - 48;
	}
	let mut input: [i32; LEN] = output;
	println!("{:?}", input.to_vec());

	for _phase in 0 .. PHASES
	{
		for (i, _) in input.iter().enumerate()
		{
			let mut total = 0;
			let mut mem = Vec::<i32>::new();
			let mut pattern = get_pattern(i + 1, &mut mem);
			for ch in input.iter()
			{
				let mul = pattern.next();
				//println!("{:?}", mul);
				total += ch * mul.expect("");
			}
			output[i] = total.abs() % 10;
		}
		input = output;
		println!("{:?}", input.to_vec());
		}
	
    //println!("{}{}{}{}{}{}{}{}", output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7]);
}
