use std::fs::File;
use std::io::prelude::*;

const ADD: u64 = 1;
const MUL: u64 = 2;
const INPUT: u64 = 3;
const OUTPUT: u64 = 4;
const JTRUE: u64 = 5;
const JFALSE: u64 = 6;
const LT: u64 = 7;
const EQ: u64 = 8;
const ARB: u64 = 9;
const END: u64 = 99;

#[derive(Debug)]
enum ProgramResult
{
	//Init,
	Response(i64),
	Request,
	Halt,
}

trait Program
{
	fn get_arg(&self, arg: usize, op: u64) -> i64;
	fn set_arg(&mut self, arg: usize, op: u64, value: i64);
	fn step(&mut self, input: ProgramResult) -> ProgramResult;
	//fn step(&self, input: ProgramResult) -> ProgramResult;
	
	fn new(code: &Vec<i64>) -> Self;
}

#[derive(Debug)]
struct ProgramState
{
	pc: usize,
	memory: Vec<i64>,
	rel_base: usize,
	//state: ProgramResult,
}

impl Program for ProgramState
{
	fn new(code: &Vec<i64>) -> ProgramState
	{
		let mut memory = code.clone();
		memory.resize(65536, 0);
		ProgramState {
			pc: 0,
			memory: memory,
			rel_base: 0,
			//state: ProgramResult::Response(0),
		}
	}

	fn get_arg(&self, arg: usize, op: u64) -> i64
	{
		let div: u64 = 10_u64.pow(arg as u32 + 1);
		match op / div % 10
		{
			1 => self.memory[(self.pc + arg) as usize],
			2 => self.memory[(self.rel_base as i32 + self.memory[(self.pc + arg) as usize] as i32) as usize],
			_ => self.memory[0 + self.memory[(self.pc + arg) as usize] as usize],
		}
	}

	fn set_arg(&mut self, arg: usize, op: u64, value: i64)
	{
		let div: u64 = 10_u64.pow(arg as u32 + 1);
		match op / div % 10
		{
			0 =>
			{
				let addr = 0 + self.memory[(self.pc + arg) as usize] as usize;
				self.memory[addr] = value;
			}
			2 =>
			{
				let addr = (self.rel_base as i32 + self.memory[(self.pc + arg) as usize] as i32) as usize;
				self.memory[addr] = value;
			}
			_ => {}
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
					let op = self.memory[self.pc] as u64;
					self.set_arg(1, op, v);
					self.pc += 2;
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
			
			let op = self.memory[self.pc] as u64;
			match op % 100_u64
			{
				ADD =>
				{
					let total = self.get_arg(1, op) + self.get_arg(2, op);
					//let dest = self.memory[self.pc + 3] as usize;
					//self.memory[dest] = total;
					self.set_arg(3, op, total);
					self.pc += 4;
				}
				MUL =>
				{
					let total = self.get_arg(1, op) * self.get_arg(2, op);
					self.set_arg(3, op, total);
					self.pc += 4;
				}
				INPUT =>
				{
					//self.pc += 1;
					return ProgramResult::Request;
				}
				OUTPUT =>
				{
					let output = self.get_arg(1, op);
					println!("{}", output);
					self.pc += 2;
					//return ProgramResult::Response(output);
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
					if arg
					{
						self.set_arg(3, op, 1);
					}
					else
					{
						self.set_arg(3, op, 0);
					}
					self.pc += 4;
				}
				EQ =>
				{
					let arg = self.get_arg(1, op) == self.get_arg(2, op);
					if arg
					{
						self.set_arg(3, op, 1);
					}
					else
					{
						self.set_arg(3, op, 0);
					}
					self.pc += 4;
				}
				ARB =>
				{
					let arg = self.get_arg(1, op);
					self.rel_base = (self.rel_base as i64 + arg) as usize;
					self.pc += 2;
					//println!("{}", self.rel_base);
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
		let mut file = File::open("../inputs/d09.txt")?;
		file.read_to_string(&mut input)?;
	}
	let input = input;

	let original = input.trim().split(',')
		.map(|x| x.parse().expect("Not a number"))
		.collect::<Vec<i64>>();
	
	let mut pA = ProgramState::new(&original);
	
	let output = pA.step(ProgramResult::Request);
	println!("{:?}", output);
	
	let output = pA.step(ProgramResult::Response(1));
	println!("{:?}", output);
	
	let mut pA = ProgramState::new(&original);
	
	let output = pA.step(ProgramResult::Request);
	println!("{:?}", output);
	
	let output = pA.step(ProgramResult::Response(2));
	println!("{:?}", output);

	Ok(())
}

