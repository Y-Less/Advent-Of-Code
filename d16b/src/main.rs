//#![feature(generators, generator_trait)]
//
//use std::ops::{Generator, GeneratorState};
//use std::pin::Pin;
use std::vec::Vec;

const PHASES: i32 = 100;
const DUP: usize = 10000;
const BASE: usize = 650;
const LEN: usize = BASE * DUP;

fn main()
{
	let original = "59776034095811644545367793179989602140948714406234694972894485066523525742503986771912019032922788494900655855458086979764617375580802558963587025784918882219610831940992399201782385674223284411499237619800193879768668210162176394607502218602633153772062973149533650562554942574593878073238232563649673858167635378695190356159796342204759393156294658366279922734213385144895116649768185966866202413314939692174223210484933678866478944104978890019728562001417746656699281992028356004888860103805472866615243544781377748654471750560830099048747570925902575765054898899512303917159138097375338444610809891667094051108359134017128028174230720398965960712";
	// let original = "19617804207202209144916044189917";

	let mut input = Vec::new();
	input.resize(LEN, 0);
	for j in 0 .. DUP
	{
		for (i, ch) in original.bytes().enumerate()
		{
			input[j * BASE + i] = ch as i32 - 48;
		}
	}

	for phase in 0 .. PHASES
	{
		//let mut output: [i32; LEN] = [0; LEN];
		let mut output = Vec::new();
		output.resize(LEN, 0);
		println!("PHASE: {}", phase + 1);
		//for (i, _) in input.iter().enumerate()
		for i in 0 .. LEN
		{
			let mut total = 0;
			let mut j = i;
			let mut e = j + i + 1;
			let mut mul = 1;
			loop
			{
				// Skip all the zeros.
				if j >= e
				{
					j = e + i + 1;
					e = j + i + 1;
					mul = -mul;
				}
				if j >= LEN
				{
					break;
				}
				total += input[j] * mul;
				j += 1;
			}
			output[i] = total.abs() % 10;
		}
		input = output;
	}
	
	let offset = 0;
    println!("{}{}{}{}{}{}{}{}", input[offset + 0], input[offset + 1], input[offset + 2], input[offset + 3], input[offset + 4], input[offset + 5], input[offset + 6], input[offset + 7]);
	let offset = (100000 * input[0] + 10000 * input[1] + 1000 * input[2] + 100 * input[3] + 10 * input[4] + input[5]) as usize;
    println!("{}{}{}{}{}{}{}{}", input[offset + 0], input[offset + 1], input[offset + 2], input[offset + 3], input[offset + 4], input[offset + 5], input[offset + 6], input[offset + 7]);
}
