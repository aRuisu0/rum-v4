use crate::rum::Vm;
use crate::dump::*;
use std::io;
use std::io::Read;

// Word coded by one least significant bit
// fn one_register(word: u32) -> usize {
// 	match bitpack::bitpack::getu(word as u64, 3, 0) {
//         // Convert the register index to a usize and return it.
//         index => index.try_into().unwrap(),
//     }
// }

// // Word coded by two least significant bits
// fn two_registers(word: u32) -> (usize, usize) {
// 	let word_u64 = word as u64;
//     match (
//         bitpack::bitpack::getu(word_u64, 3, 3),
//         bitpack::bitpack::getu(word_u64, 3, 0),
//     ) {
//         // Convert the register indices to usizes and return them as a tuple.
//         (b, c) => (b.try_into().unwrap(), c.try_into().unwrap()),
//     }

// }
 
// // Word coded by three least significant bits
// fn three_registers(word: u32) -> (usize, usize, usize) {
// 	let word_u64 = word as u64;
//     match (
//         bitpack::bitpack::getu(word_u64, 3, 6),
//         bitpack::bitpack::getu(word_u64, 3, 3),
//         bitpack::bitpack::getu(word_u64, 3, 0),
//     ) {
//         // Convert the register indices to usizes and return them as a tuple.
//         (a, b, c) => (
//             a.try_into().unwrap(),
//             b.try_into().unwrap(),
//             c.try_into().unwrap(),
//         ),
//     }
// }

// Conditional Load Operator
pub fn cond_move(vm: &mut Vm, word: u32) {
	// Conditional Load
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	
	if vm.registers[c] != 0 {
		vm.registers[a] = vm.registers[b];
	}
}

// Segmented Load Operator
// This function is using indexing to access the value in the memory array at the indices specified by the b and c registers.
pub fn seg_load(vm: &mut Vm, word: u32) {
	// Segmented Load
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	vm.registers[a] = vm.memory[vm.registers[b] as usize][vm.registers[c] as usize];
}

// Segmented Store Operator
pub fn seg_store(vm: &mut Vm, word: u32) {
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));

	 // Use indexing to set the value in the memory array at the indices specified by the a and b registers.
	vm.memory[vm.registers[a] as usize][vm.registers[b] as usize] = vm.registers[c];
}

// Add Operator  
//This function adds the values stored in the bth and cth elements of the vm object's 
//registers array and stores the result in the ath element of the array.
pub fn add(vm: &mut Vm, word: u32) {
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	vm.registers[a] = ((vm.registers[b] as u64 + vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Multiply Operator
//This function multiplys the values stored in the bth and cth elements of the vm object's 
//registers array and stores the result in the ath element of the array.
pub fn mul(vm: &mut Vm, word: u32) {
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	vm.registers[a] = ((vm.registers[b] as u64 * vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Divide Operator
// this code performs a division operation on the values 
// stored in two of the virtual machine's registers, storing the result in a third register.
pub fn div(vm: &mut Vm, word: u32) {
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	vm.registers[a] = vm.registers[b] / vm.registers[c];	
}

// Bitwise NAND Operator
// Takes a vm object and an integer as arguments, extracts the values
// of three registers from the integer, performs a bitwise AND 
// operation on two of the register values, negates the result, and stores it in the third register.
pub fn nand(vm: &mut Vm, word: u32) {
	let (a, b, c) = (get(&RA, word), get(&RB, word), get(&RC, word));
	vm.registers[a] = !(vm.registers[b] & vm.registers[c]);	
}

// Halt Operator
pub fn halt(_vm: &mut Vm) {
	std::process::exit(0);
}

// Map Segment Operator
// Function for managing the allocation of memory segments in a 
// virtual machine. It allows for the creation of new segments or the re-use of previously unmapped segments
pub fn map_seg(vm: &mut Vm, word: u32) {
	let (b, c) = (get(&RB, word), get(&RC, word));
	if vm.unmapped_segs.len() != 0 {
		let segment_number = vm.unmapped_segs.pop().unwrap();
		vm.memory[segment_number] = vec![0; vm.registers[c] as usize];
		vm.registers[b] = segment_number as u32;
	} 
	else {
		vm.max_mapped_seg += 1;
		vm.memory.push(vec![0; vm.registers[c] as usize]);
		vm.registers[b] = vm.max_mapped_seg as u32;
	}
}

// Unmap Segment Operator
// Used for managing the memory segments in a 
// virtual machine for unmapping or removing a memory segment fromm memory
pub fn unmap_seg(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);

	vm.memory[vm.registers[c] as usize].clear();
	vm.unmapped_segs.push(vm.registers[c].try_into().unwrap());
}

// Output Operator
//Uses the word to extract a single register value 
//from the vm's registers array and prints 
//the char representation of the register
pub fn output(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);
	print!("{}", vm.registers[c] as u8 as char);
}

//Input Operator
//The function assigns the value of value to the c register of the vm struct
pub fn input(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);
    let mut buffer: [u8; 1] = [0; 1];
	let num = io::stdin().read(&mut buffer);
    let value = match num {
        Ok(byte) => byte as u32,
        Err(_) => !0_u32
    };
	vm.registers[c] = value;
}

// Load Program Operator
pub fn load_prog(vm: &mut Vm, word: u32) {
	let (b, c) = (get(&RB, word), get(&RC, word));

	if vm.registers[b] != 0 {
		vm.memory[0] = vm.memory[vm.registers[b] as usize].clone();
    }
	vm.prog_count = vm.registers[c];
}

// Load Value
pub fn load_val(vm: &mut Vm, word: u32) {
	let value = bitpack::bitpack::getu(word as u64, 25, 0) as u32;
	let a = bitpack::bitpack::getu(word as u64, 3, 25);
	vm.registers[a as usize] = value;
}