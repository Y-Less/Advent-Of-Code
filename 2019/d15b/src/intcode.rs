use std::fs::File;
use std::io::prelude::*;

pub const ADD: u64 = 1;
pub const MUL: u64 = 2;
pub const INPUT: u64 = 3;
pub const OUTPUT: u64 = 4;
pub const JTRUE: u64 = 5;
pub const JFALSE: u64 = 6;
pub const LT: u64 = 7;
pub const EQ: u64 = 8;
pub const ARB: u64 = 9;
pub const END: u64 = 99;

#[derive(Debug)]
pub enum ProgramResult
{
	//Init,
	Response(i64),
	Request,
	Halt,
}

pub trait Program
{
	fn get_arg(&self, arg: usize, op: u64) -> i64;
	fn set_arg(&mut self, arg: usize, op: u64, value: i64);
	fn step(&mut self, input: ProgramResult) -> ProgramResult;
	
	fn new(code: &Vec<i64>) -> Self;
	fn load(fname: &str) -> Self;
}

#[derive(Debug)]
pub struct ProgramState
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

	fn load(fname: &str) -> ProgramState
	{
		let mut input = String::new();
		let mut file = File::open(fname).expect("Could not read program file");
		file.read_to_string(&mut input).expect("Could not read program file");

		let original = input.trim().split(',')
			.map(|x| x.parse().expect("Not a number"))
			.collect::<Vec<i64>>();
		
		ProgramState::new(&original)
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
