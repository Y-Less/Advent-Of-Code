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
const LEN: usize = 650;

fn main()
{

	let input = "59776034095811644545367793179989602140948714406234694972894485066523525742503986771912019032922788494900655855458086979764617375580802558963587025784918882219610831940992399201782385674223284411499237619800193879768668210162176394607502218602633153772062973149533650562554942574593878073238232563649673858167635378695190356159796342204759393156294658366279922734213385144895116649768185966866202413314939692174223210484933678866478944104978890019728562001417746656699281992028356004888860103805472866615243544781377748654471750560830099048747570925902575765054898899512303917159138097375338444610809891667094051108359134017128028174230720398965960712";
	let mut output: [i32; LEN] = [0; LEN];
	for (i, ch) in input.bytes().enumerate()
	{
		output[i] = ch as i32 - 48;
	}
	let mut input: [i32; LEN] = output;

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
		//println!("{:?}", output);
		input = output;
	}
	
    println!("{}{}{}{}{}{}{}{}", output[0], output[1], output[2], output[3], output[4], output[5], output[6], output[7]);
}
