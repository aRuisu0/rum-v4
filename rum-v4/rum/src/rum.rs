use crate::instructs::*;
use std::env;
use std::io::Read;

// This public structure of the virtual machine will contain the registers and memory of the segments
// of the opcode instructions and will contain the counter for the program when machine is running
pub struct Vm {
	pub registers: Vec<u32>,
	pub memory: Vec<Vec<u32>>,
	pub unmapped_segs: Vec<usize>,
	pub max_mapped_seg: usize,
	pub prog_count: u32,
	// pub running: bool
}

// Virtual machine that will start to boot and set memory and increment counter 
impl Vm {

	pub fn new() -> Self {
		Vm {
			registers: vec![0_u32; 8],
			memory: vec![],
			unmapped_segs: vec![],
			max_mapped_seg: 0,
			prog_count: 0
		}
	}

	pub fn boot(&mut self) {
		// let instruction = self.memory[0][self.prog_count as usize];
		// self.prog_count += 1;
		// instruction

		/*
		let instruction = self.memory[0][self.program_count as usize];
		self.program_count += 1;
		instruction
		*/
		let args: Vec<String> = env::args().collect();
	    let input: Option<&str>;
	    
	    if args.len() == 2 {
	    	input = Some(&args[1]);
	    } else {
	    	input = None;
	    }
	    
	    //Code provided by Professor Daniels from the rumdump lab
	    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
			None => Box::new(std::io::BufReader::new(std::io::stdin())),
			Some(filename) => Box::new(std::io::BufReader::new(
				std::fs::File::open(filename).unwrap(),
			)),
		};
		
			let mut buf = Vec::<u8>::new();
			raw_reader.read_to_end(&mut buf).unwrap();
		
			let instructions: Vec<u32> = buf
				.chunks_exact(4)
				.map(|x| u32::from_be_bytes(x.try_into().unwrap()))
				.collect();
			self.memory.push(instructions); 
	}

	pub fn run(&mut self) {
		loop {
			let instruction = self.get();
			self.execute(instruction);
		}
	}

	fn get(&mut self) -> u32 {
		let instruction = self.memory[0][self.prog_count as usize];
		self.prog_count += 1;
		instruction
	}

	pub fn execute(&mut self, word: u32){

		// Decore our file input from bitpack::getu that will retrieve an unsigned value 
		//from `word`, represented by `width` bits beginning at least-significant bit `lsb`.
		// let opcode: u8 = bitpack::bitpack::getu(word.into(), 4, 28).try_into().unwrap();
		// let word_u32: u32 = word.try_into().unwrap();

// 		The >> operator shifts the bits of word to the right by 28 places, effectively moving the opcode bits to the rightmost position in the 
//resulting value. The & operator then performs a bitwise AND operation with the value (1 << 4) - 1, 
//which is a mask that has the first 4 bits set to 1 and the rest set to 0. This mask is used to isolate the first 4 bits of the word, which contain the opcode.

// The resulting value is the opcode extracted from the word.
		let opcode = (word >> 28) & (1 << 4) - 1;

		// Excecute our Opcode conditions with the word of u32 bit
		match opcode {
			0 =>  cond_move(self, word),
			1 =>  seg_load(self, word),
			2 =>  seg_store(self, word),
			3 =>  add(self, word),
			4 =>  mul(self, word),
			5 =>  div(self, word),
			6 =>  nand(self, word),
			7 =>  halt(self),
			8 =>  map_seg(self, word),
			9 =>  unmap_seg(self, word),
			10 => output(self, word),
			11 => input(self, word),
			12 => load_prog(self, word),
			13 => load_val(self, word),
			 _ => panic!("Error")

		};
	}
} 