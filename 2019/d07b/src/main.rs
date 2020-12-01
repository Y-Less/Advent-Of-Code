use std::fs::File;
use std::io::prelude::*;
use permutohedron::LexicalPermutation;

//use std::iter;
//use std::vec;

const ADD: u32 = 1;
const MUL: u32 = 2;
const INPUT: u32 = 3;
const OUTPUT: u32 = 4;
const JTRUE: u32 = 5;
const JFALSE: u32 = 6;
const LT: u32 = 7;
const EQ: u32 = 8;
const END: u32 = 99;

#[derive(Debug)]
enum ProgramResult
{
	//Init,
	Response(i32),
	Request,
	Halt,
}

trait Program
{
	fn get_arg(&self, arg: usize, op: u32) -> i32;
	fn step(&mut self, input: ProgramResult) -> ProgramResult;
	//fn step(&self, input: ProgramResult) -> ProgramResult;
	
	fn new(code: &Vec<i32>) -> Self;
}

#[derive(Debug)]
struct ProgramState
{
	pc: usize,
	memory: Vec<i32>,
	//state: ProgramResult,
}

impl Program for ProgramState
{
	fn new(code: &Vec<i32>) -> ProgramState
	{
		ProgramState {
			pc: 0,
			memory: code.clone(),
			//state: ProgramResult::Response(0),
		}
	}

	fn get_arg(&self, arg: usize, op: u32) -> i32
	{
		let div = 10_u32.pow(arg as u32 + 1);
		match op / div % 10
		{
			1 => self.memory[(self.pc + arg) as usize],
			_ => self.memory[self.memory[(self.pc + arg) as usize] as usize],
		}
	}

	fn step(&mut self, input: ProgramResult) -> ProgramResult
	{
		let len = self.memory.len();

		if self.pc < len
		{
			match input
			{
				ProgramResult::Halt =>
				{
					// Do nothing.
					return ProgramResult::Halt
				}
				ProgramResult::Response(v) =>
				{
					let dest = self.memory[self.pc] as usize;
					self.memory[dest] = v;
					self.pc += 1;
				}
				_ =>
				{
					// Do nothing.
				}
			}
		}

		loop
		{
			if self.pc >= len
			{
				break;
			}
			
			let op = self.memory[self.pc] as u32;
			match op % 100
			{
				ADD =>
				{
					let total = self.get_arg(1, op) + self.get_arg(2, op);
					let dest = self.memory[self.pc + 3] as usize;
					self.memory[dest] = total;
					self.pc += 4;
				}
				MUL =>
				{
					let total = self.get_arg(1, op) * self.get_arg(2, op);
					let dest = self.memory[self.pc + 3] as usize;
					self.memory[dest] = total;
					self.pc += 4;
				}
				INPUT =>
				{
					self.pc += 1;
					return ProgramResult::Request;
				}
				OUTPUT =>
				{
					let output = self.get_arg(1, op);
					//println!("{}", output);
					self.pc += 2;
					return ProgramResult::Response(output);
				}
				JTRUE =>
				{
					let arg = self.get_arg(1, op);
					let pc = self.get_arg(2, op);
					if arg != 0
					{
						self.pc = pc as usize;
					}
					else
					{
						self.pc += 3;
					}
				}
				JFALSE =>
				{
					let arg = self.get_arg(1, op);
					let pc = self.get_arg(2, op);
					if arg == 0
					{
						self.pc = pc as usize;
					}
					else
					{
						self.pc += 3;
					}
				}
				LT =>
				{
					let arg = self.get_arg(1, op) < self.get_arg(2, op);
					let dest = self.memory[self.pc + 3] as usize;
					if arg
					{
						self.memory[dest] = 1;
					}
					else
					{
						self.memory[dest] = 0;
					}
					self.pc += 3;
				}
				EQ =>
				{
					let arg = self.get_arg(1, op) == self.get_arg(2, op);
					let dest = self.memory[self.pc + 3] as usize;
					if arg
					{
						self.memory[dest] = 1;
					}
					else
					{
						self.memory[dest] = 0;
					}
					self.pc += 3;
				}
				END =>
				{
					//println!("{}, {}", self.pc, len);
					//self.pc += 1;
					//break;
					self.pc = len;
					return ProgramResult::Halt;
				}
				_ =>
				{
					self.pc += 1;
				}
			}
		}
		
		ProgramResult::Halt
	}
}

fn main() -> std::io::Result<()>
{
	let mut input = String::new();
	{
		let mut file = File::open("../inputs/d07.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let original = input.trim().split(',')
		.map(|x| x.parse().expect("Not a number"))
		.collect::<Vec<i32>>();

	//let mut prog = original.clone();
	//
	//run(&mut prog, &mut (0..5));

	// Test permutation.
	let mut settings = [5, 6, 7, 8, 9]; // `vec!(5, 6, 7, 8, 9)`

	//let mut v = Vec::new();
	//v.push(0);
	//v.push(1);
	//v.push(3);
	//v.push(2);
	//v.push(4);

	let mut highest = 0;

	loop
	{
		let mut pA = ProgramState::new(&original);
		let mut pB = ProgramState::new(&original);
		let mut pC = ProgramState::new(&original);
		let mut pD = ProgramState::new(&original);
		let mut pE = ProgramState::new(&original);
		
		pA.step(ProgramResult::Request);
		pB.step(ProgramResult::Request);
		pC.step(ProgramResult::Request);
		pD.step(ProgramResult::Request);
		pE.step(ProgramResult::Request);

		let sv = settings.to_vec();
		//println!("{:?}", pA.step(ProgramResult::Response(sv[0])));
		//println!("{:?}", pB.step(ProgramResult::Response(sv[1])));
		//println!("{:?}", pC.step(ProgramResult::Response(sv[2])));
		//println!("{:?}", pD.step(ProgramResult::Response(sv[3])));
		//println!("{:?}", pE.step(ProgramResult::Response(sv[4])));
		pA.step(ProgramResult::Response(sv[0]));
		pB.step(ProgramResult::Response(sv[1]));
		pC.step(ProgramResult::Response(sv[2]));
		pD.step(ProgramResult::Response(sv[3]));
		pE.step(ProgramResult::Response(sv[4]));
		println!("{:?}", sv);
		
		let mut input = 0;
		
		let mut dA = false;
		let mut dB = false;
		let mut dC = false;
		let mut dD = false;
		let mut dE = false;

		let mut cA = 0;
		let mut cB = 0;
		let mut cC = 0;
		let mut cD = 0;
		let mut cE = 0;

		loop
		{
			//println!("{} {} {} {} {}", dA, dB, dC, dD, dE);
			//println!("{} {} {} {} {}", cA, cB, cC, cD, cE);
			if !dA
			{
				match pA.step(ProgramResult::Response(input))
				{
					ProgramResult::Request =>
					{
						//println!("{} {} {} {} {}", dA, dB, dC, dD, dE);
						// I don't know WHY this works...  It shouldn't - the code is waiting on
						// input without ever producing anything new.
						break;//panic!("Double Request A");
					}
					ProgramResult::Response(v) =>
					{
						input = v;
						pA.step(ProgramResult::Request);
					}
					ProgramResult::Halt =>
					{
						println!("Halted");
						dA = true;
					}
				}
				cA += 1
			}
			if !dB
			{
				match pB.step(ProgramResult::Response(input))
				{
					ProgramResult::Request =>
					{
						// I don't know WHY this works...  It shouldn't - the code is waiting on
						// input without ever producing anything new.
						break;//panic!("Double Request B");
					}
					ProgramResult::Response(v) =>
					{
						input = v;
						pB.step(ProgramResult::Request);
					}
					ProgramResult::Halt =>
					{
						dB = true;
					}
				}
				cB += 1
			}
			if !dC
			{
				match pC.step(ProgramResult::Response(input))
				{
					ProgramResult::Request =>
					{
						// I don't know WHY this works...  It shouldn't - the code is waiting on
						// input without ever producing anything new.
						break;//panic!("Double Request C");
					}
					ProgramResult::Response(v) =>
					{
						input = v;
						pC.step(ProgramResult::Request);
					}
					ProgramResult::Halt =>
					{
						dC = true;
					}
				}
				cC += 1
			}
			if !dD
			{
				match pD.step(ProgramResult::Response(input))
				{
					ProgramResult::Request =>
					{
						// I don't know WHY this works...  It shouldn't - the code is waiting on
						// input without ever producing anything new.
						break;//panic!("Double Request D");
					}
					ProgramResult::Response(v) =>
					{
						input = v;
						pD.step(ProgramResult::Request);
					}
					ProgramResult::Halt =>
					{
						dD = true;
					}
				}
				cD += 1
			}
			if !dE
			{
				match pE.step(ProgramResult::Response(input))
				{
					ProgramResult::Request =>
					{
						panic!("Double Request E");
					}
					ProgramResult::Response(v) =>
					{
						input = v;
						pE.step(ProgramResult::Request);
					}
					ProgramResult::Halt =>
					{
						dE = true;
					}
				}
				cE += 1
			}
			if dA && dB && dC && dD && dE
			{
				if input > highest
				{
					highest = input;
				}
				break;
			}
		}
		if !settings.next_permutation()
		{
			break;
		}
	}

	println!("{}", highest);

	Ok(())
}

